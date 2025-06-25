use std::sync::Arc;

use k8s_openapi::serde_json;
use log::warn;

use crate::{server::http::ui::get_stats::Stats, SharedInternalDatabase};

pub fn stats_answer(internal_database: SharedInternalDatabase) -> Vec<u8> {
    let size = match internal_database.lock() {
        Ok(data) => data.len(),
        Err(poisoned) => {
            warn!("Mutex poisoned, recovering: {:?}", poisoned);
            0
        }
    };

    let toto = Arc::clone(&internal_database);

    let stats = Stats {
        memory_mb: calculate_memory_usage(toto),
        key_count: size,
    };

    let json = serde_json::to_string(&stats).unwrap();
    json.as_bytes().to_vec()
}

pub fn calculate_memory_usage(db: SharedInternalDatabase) -> f64 {
    deep_size_of_arc_mutex_mb(db)
}

fn deep_size_of_arc_mutex_mb(map: SharedInternalDatabase) -> f64 {
    if let Ok(map) = map.lock() {
        let mut size_bytes = std::mem::size_of_val(&*map);
        for (k, v) in map.iter() {
            size_bytes += k.capacity();
            size_bytes += v.capacity();
        }
        size_bytes as f64 / (1024.0 * 1024.0)
    } else {
        warn!("Cannot determine size, lock not acquired");
        0.0 // default or fallback
    }
}
