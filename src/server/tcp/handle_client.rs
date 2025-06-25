use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::str;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use log::debug;
use log::error;
use log::info;
use log::warn;

use crate::server::tcp::add::send_command_add_to_all_peers;
use crate::server::tcp::commands::flush_all::send_command_flush_all_to_all_peers;
use crate::server::tcp::responses::identification::identification_answer;
use crate::server::tcp::responses::stats::stats_answer;
use crate::SharedInternalDatabase;
use crate::SharedPeers;
use crate::SharedTimingStats;

pub fn handle_client(
    mut stream: TcpStream,
    db: SharedInternalDatabase,
    peers: SharedPeers,
    my_name: String,
    my_address: String,
    time_stats: SharedTimingStats,
) {
    let mut data = [0u8; 10000];

    loop {
        match stream.read(&mut data) {
            Ok(0) => break, // client closed

            Ok(size) => {
                let input = match str::from_utf8(&data[..size]) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("Invalid UTF-8: {}", e);
                        continue;
                    }
                };

                debug!("Request: {}", input);

                // MY PART
                if input.contains("|") {
                    let parts: Vec<&str> = input.split("|").collect();

                    let peers_clone = Arc::clone(&peers);
                    let db_clone = Arc::clone(&db);

                    let response = match parts[0] {
                        "ADD" => {
                            let properties: Vec<&str> = parts[1].split(",").collect();

                            let key: Vec<&str> = properties[0].split("=").collect();
                            let value: Vec<&str> = properties[1].split("=").collect();
                            let date: Vec<&str> = properties[2].split("=").collect();

                            match db.lock() {
                                Ok(mut data) => {
                                    data.insert(key[1].to_string(), value[1].to_string());
                                }
                                Err(poisoned) => {
                                    warn!("Mutex poisoned, recovering: {:?}", poisoned);
                                }
                            };

                            let now = Utc::now();

                            let sender_date: DateTime<Utc> = DateTime::parse_from_rfc3339(date[1])
                                .unwrap()
                                .with_timezone(&Utc);
                            let diff = now - sender_date;

                            let micros_str = diff
                                .num_microseconds()
                                .map(|micros| format!("{}µs", micros))
                                .unwrap_or_else(|| "Too large".to_string());

                            {
                                let mut stats: std::sync::MutexGuard<
                                    '_,
                                    crate::timing::TimingStats,
                                > = time_stats.lock().unwrap();
                                stats.add_sample(
                                    diff.num_microseconds()
                                        .map(|m| m)
                                        .unwrap_or_else(|| -1)
                                        .try_into()
                                        .unwrap(),
                                );
                                let avg = stats.average_micros().unwrap_or(0.0);
                                println!(
                                    "Command took {}, average so far {:.2} μs",
                                    micros_str, avg
                                );
                            }

                            "OK".to_string().as_bytes().to_vec()
                        }
                        "IDENTIFICATION" => identification_answer(
                            peers_clone,
                            db_clone,
                            parts,
                            my_name.clone(),
                            my_address.clone(),
                        ),
                        "STATS" => stats_answer(db_clone),
                        "DISCONNECT" => {
                            let properties: Vec<&str> = parts[1].split(",").collect();
                            let name: Vec<&str> = properties[0].split("=").collect();

                            peers.lock().unwrap().remove(name[1]);

                            "OK".to_string().as_bytes().to_vec()
                        }
                        "COPY" => {
                            let parts: Vec<&str> = input.split("|").collect();

                            for part in parts {
                                if part != "COPY" {
                                    let properties: Vec<&str> = part.split(",").collect();
                                    let key: Vec<&str> = properties[0].split("=").collect();
                                    let value: Vec<&str> = properties[1].split("=").collect();

                                    db_clone
                                        .lock()
                                        .unwrap()
                                        .insert(key[1].to_string(), value[0].to_string());
                                }
                            }

                            "OK".to_string().as_bytes().to_vec()
                        }
                        "FLUSHALL" => {
                            if let Ok(mut map) = db.lock() {
                                map.clear();
                                info!("Database has been flushed");
                            } else {
                                error!("Failed to acquire lock on the database");
                            }

                            "OK".to_string().as_bytes().to_vec()
                        }
                        command => {
                            error!("Command {} has not been implemented", command);
                            todo!();
                        }
                    };

                    if let Err(e) = stream.write_all(&response) {
                        error!("Write failed: {}", e);
                        break;
                    }
                }

                // REDIS PART

                let mut refactored_mode = false;

                let commands: Vec<&str> =
                    input.split("*4").filter(|s| !s.trim().is_empty()).collect();
                for command in commands {
                    let lines: Vec<&str> = command
                        .split("\r\n")
                        .filter(|s| !s.trim().is_empty())
                        .collect();

                    let first_keyword = lines[1];

                    debug!("COMMAND");
                    for line in lines {
                        debug!("LINE {}", line);
                    }

                    if first_keyword == "CLIENT" {
                        let answer = "+OK\r\n".to_string();
                        if let Err(e) = stream.write_all(answer.as_bytes()) {
                            error!("Write failed: {}", e);
                            break;
                        }
                        refactored_mode = true;
                    }
                }

                if refactored_mode {
                    continue;
                }

                // Basic parser for RESP array
                let lines: Vec<&str> = input.split("\r\n").collect();
                if lines.len() < 3 {
                    continue;
                }

                let command = lines[2].to_uppercase();
                let peers_clone = Arc::clone(&peers);

                let response = match command.as_str() {
                    "CLIENT" | "SETINFO" | "LIB-VER" | "LIB-NAME" => "+OK\r\n+OK\r\n".to_string(),
                    "PING" => "+PONG\r\n".to_string(),
                    "ECHO" => {
                        if lines.len() >= 5 {
                            format!("+{}\r\n", lines[4])
                        } else {
                            "-ERR wrong number of arguments for 'echo'\r\n".to_string()
                        }
                    }
                    "SET" => {
                        if lines.len() >= 7 {
                            let key = lines[4].to_string();
                            let value = lines[6].to_string();

                            let k = key.clone();
                            let v = value.clone();

                            db.lock().unwrap().insert(key, value);

                            send_command_add_to_all_peers(peers_clone, k, v);

                            "+OK\r\n".to_string()
                        } else {
                            "-ERR wrong number of arguments for 'set'\r\n".to_string()
                        }
                    }
                    "GET" => {
                        if lines.len() >= 5 {
                            let key = lines[4].to_string();

                            let answer = match db.lock() {
                                Ok(data) => match data.get(&key) {
                                    Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
                                    None => "$-1\r\n".to_string(),
                                },
                                Err(poisoned) => {
                                    warn!("Mutex poisoned, recovering: {:?}", poisoned);
                                    "$-1\r\n".to_string()
                                }
                            };

                            println!("here, {}", answer);
                            answer
                        } else {
                            "-ERR wrong number of arguments for 'get'\r\n".to_string()
                        }
                    }
                    "FLUSHALL" => {
                        if let Ok(mut map) = db.lock() {
                            map.clear();
                            info!("Database has been flushed");
                        } else {
                            error!("Failed to acquire lock on the database");
                        }

                        send_command_flush_all_to_all_peers(peers_clone);

                        "+OK\r\n".to_string()
                    }
                    _ => "-ERR unknown command\r\n".to_string(),
                };

                if let Err(e) = stream.write_all(response.as_bytes()) {
                    error!("Write failed: {}", e);
                    break;
                }
            }
            Err(e) => {
                error!("Read error: {}", e);
                break;
            }
        }
    }
}
