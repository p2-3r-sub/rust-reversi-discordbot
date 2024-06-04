use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[path = "./reversi/mod.rs"]
pub mod reversi;
use reversi::stats::RStats;

pub struct GlobalReversiStats;

impl TypeMapKey for GlobalReversiStats {
    type Value = Arc<Mutex<HashMap<u64, RStats>>>;
}
