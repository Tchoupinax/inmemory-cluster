mod env;
pub mod server;
pub mod timing;

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use crate::timing::TimingStats;

pub type SharedPeers = Arc<Mutex<BTreeMap<String, String>>>;
pub type SharedInternalDatabase = Arc<Mutex<HashMap<String, String>>>;
pub type SharedTimingStats = Arc<Mutex<TimingStats>>;
#[derive(Clone)]
pub struct State {
    pub hostname: String,
    pub reacheable_url: String,
}
