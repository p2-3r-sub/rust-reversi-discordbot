use serenity::all::{Message, UserId};

use super::gomoku::QuantumGomoku;

#[derive(Debug)]
pub struct QGStatsUser {
    pub id: UserId,
    pub choiced_alphabet: Option<String>,
    pub choiced_number: Option<String>,
}

impl QGStatsUser {
    pub fn new(user_id: UserId) -> Self {
        Self {
            id: user_id,
            choiced_alphabet: None,
            choiced_number: None,
        }
    }
}

#[derive(Debug)]
pub struct QGStats {
    pub gomoku: QuantumGomoku,
    pub message: Option<Message>,

    pub black_user: Option<QGStatsUser>,
    pub white_user: Option<QGStatsUser>,
}

impl QGStats {
    pub fn new() -> Self {
        let gomoku = QuantumGomoku::new();

        Self {
            gomoku,
            message: None,

            black_user: None,
            white_user: None,
        }
    }
}
