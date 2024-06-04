use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

pub fn get_token(file_path: &str) -> Result<String> {
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let config: Token = serde_json::from_reader(reader).unwrap();

    Ok(config.token)
}
