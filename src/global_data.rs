use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::reversi;

pub struct GlobalReversiStats;

impl TypeMapKey for GlobalReversiStats {
    type Value = Arc<Mutex<HashMap<u64, reversi::stats::RStats>>>;
}
