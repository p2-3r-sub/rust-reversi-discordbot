use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{quantum_gomoku, reversi};

pub struct GlobalReversiStats;

impl TypeMapKey for GlobalReversiStats {
    type Value = Arc<Mutex<HashMap<u64, reversi::stats::RStats>>>;
}

pub struct GlobalQuantumGomokuStats;

impl TypeMapKey for GlobalQuantumGomokuStats {
    type Value = Arc<Mutex<HashMap<u64, quantum_gomoku::stats::QGStats>>>;
}
