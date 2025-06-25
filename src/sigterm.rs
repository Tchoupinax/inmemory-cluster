use std::collections::BTreeMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self};
use std::{sync::atomic::Ordering, time};

use log::debug;
// https://dev.to/talzvon/handling-unix-kill-signals-in-rust-55g6
// This is just a collection of ints that represent kill signals.
// More specifically, they are the common kill signals used to
// terminate a program
// You can do println!("{:?}", TERM_SIGNALS) to see them
// They are just SIGINT(2), SIGTERM(15) and SIGQUIT(3)
use signal_hook::consts::TERM_SIGNALS;
// Module that sets boolean flags when kill signal is received
use signal_hook::flag;

use crate::env::Environment;
use crate::server::tcp::commands::disconnect_from_peers;

pub fn sigterm_handler(
    peers: Arc<Mutex<BTreeMap<String, String>>>,
    hostname: String,
    config_environment: Environment,
) -> std::thread::JoinHandle<()> {
    // A special boolean that can be used across threads
    // It will be passed to flag::register, which will
    // set it to true the first time a kill signal is received
    let term_now = Arc::new(AtomicBool::new(false));

    // Register all kill signals
    // Note: You COULD specify other, specific kill signals here
    // rather than the 3 in TERM_SIGNALS. You just need a vector
    // of constants from signal_hook::consts::signal
    for sig in TERM_SIGNALS {
        // When terminated by a second term signal, exit with exit code 1.
        // This will do nothing the first time (because term_now is false).
        let _ = flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now));
        // But this will "arm" the above for the second time, by setting it to true.
        // The order of registering these is important, if you put this one first, it will
        // first arm and then terminate ‒ all in the first round.
        let _ = flag::register(*sig, Arc::clone(&term_now));
    }

    thread::spawn(move || loop {
        // Main process that does work until term_now has been set
        // to true by flag::register
        while !term_now.load(Ordering::Relaxed) {
            match config_environment {
                Environment::Prod => debug!("Waiting for signal"),
                Environment::Dev => {}
            }
            thread::sleep(time::Duration::from_secs(1));
        }

        // If we ended up here, the loop above exited because of a kill signal
        println!("\nCtrl+C received. Graceful shutdown started... (Press again to force kill, it's not recommended)");

        // This simulates a long cleanup operation
        // If you wait this long, the program will exit
        // If you hit Ctrl+C again before this is done, flag::register_conditional_shutdown will kill
        // the process without waiting for it to finish. This means double Ctrl+C kills the process
        // immediately
        disconnect_from_peers(&peers, hostname);

        thread::sleep(time::Duration::from_secs(1));

        std::process::exit(0);
    })
}
