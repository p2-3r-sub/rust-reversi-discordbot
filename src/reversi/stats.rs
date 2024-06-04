use serenity::all::{Message, UserId};

use super::reversi::Reversi;

#[derive(Debug)]
pub struct RStatsUser {
    pub id: UserId,
    pub choiced_alphabet: Option<String>,
    pub choiced_number: Option<String>,
}

impl RStatsUser {
    pub fn new(user_id: UserId) -> Self {
        Self {
            id: user_id,
            choiced_alphabet: None,
            choiced_number: None,
        }
    }
}

#[derive(Debug)]
pub struct RStats {
    pub reversi: Reversi,
    pub message: Option<Message>,

    pub black_user: Option<RStatsUser>,
    pub white_user: Option<RStatsUser>,
}

impl RStats {
    pub fn new() -> Self {
        let reversi = Reversi::new();

        Self {
            reversi,
            message: None,

            black_user: None,
            white_user: None,
        }
    }
}
