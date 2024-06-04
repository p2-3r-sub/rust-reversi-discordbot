#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stone {
    NONE,
    WHITE,
    BLACK,
}

#[allow(unused)]
struct SideStones {
    stones: Vec<Stone>,
    move_row: i32,
    move_column: i32,
}

impl SideStones {
    fn new(move_row: i32, move_column: i32) -> Self {
        Self {
            stones: vec![],
            move_row: move_row,
            move_column: move_column,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Reversi {
    pub board: [[Stone; 8]; 8],
    pub turn_stone: Stone,
}

#[allow(unused)]
impl Reversi {
    pub fn new() -> Self {
        let mut reversi = Self {
            board: [[Stone::NONE; 8]; 8],
            turn_stone: Stone::BLACK,
        };

        reversi.board[3][3] = Stone::WHITE;
        reversi.board[4][4] = Stone::WHITE;

        reversi.board[3][4] = Stone::BLACK;
        reversi.board[4][3] = Stone::BLACK;

        return reversi;
    }

    pub fn switch_turn(&mut self) {
        if self.turn_stone == Stone::BLACK {
            self.turn_stone = Stone::WHITE
        } else {
            self.turn_stone = Stone::BLACK
        }
    }

    fn sidestones_array(&self, row: i32, column: i32) -> [SideStones; 8] {
        let lower = SideStones::new(1, 0);
        let upper = SideStones::new(-1, 0);

        let right = SideStones::new(0, 1);
        let upper_right = SideStones::new(-1, 1);
        let lower_right = SideStones::new(1, 1);

        let left = SideStones::new(0, -1);
        let upper_left = SideStones::new(-1, -1);
        let lower_left = SideStones::new(1, -1);

        let mut sidestones_array = [
            lower,
            upper,
            right,
            upper_right,
            lower_right,
            left,
            upper_left,
            lower_left,
        ];

        if self.board[row as usize][column as usize] != Stone::NONE {
            return sidestones_array;
        }

        for i in &mut sidestones_array {
            let mut cur_row = row;
            let mut cur_column = column;

            loop {
                cur_row += i.move_row;
                cur_column += i.move_column;

                let stone = self.square_state(cur_row, cur_column);

                match stone {
                    Ok(stone) => i.stones.push(stone),
                    Err(_) => break,
                }
            }
        }

        return sidestones_array;
    }

    pub fn can_place_square(&self, row: i32, column: i32, self_color: Stone) -> bool {
        let rival_color = {
            if self_color == Stone::BLACK {
                Stone::WHITE
            } else {
                Stone::BLACK
            }
        };

        let sidestones_array = self.sidestones_array(row, column);

        for i in sidestones_array {
            let stone_vec = i.stones;

            if stone_vec.len() == 0 {
                continue;
            } else if stone_vec[0] != rival_color {
                continue;
            }

            let first_self_stone = {
                match stone_vec.iter().position(|&x| x == self_color) {
                    Some(index) => index,
                    None => continue,
                }
            };

            if stone_vec[..first_self_stone].contains(&Stone::NONE) {
                continue;
            }

            return true;
        }

        return false;
    }

    pub fn do_place(&mut self, row: i32, column: i32, self_color: Stone) -> Result<String, String> {
        if !(self.can_place_square(row, column, self_color)) {
            return Err("CannotPlaced".to_string());
        }

        let rival_color = {
            if self_color == Stone::BLACK {
                Stone::WHITE
            } else {
                Stone::BLACK
            }
        };

        let sidestones_array = self.sidestones_array(row, column);

        for i in sidestones_array {
            let stone_vec = i.stones;

            if stone_vec.len() == 0 {
                continue;
            } else if stone_vec[0] != rival_color {
                continue;
            }

            let first_self_stone = {
                match stone_vec.iter().position(|&x| x == self_color) {
                    Some(index) => index,
                    None => continue,
                }
            };

            if stone_vec[..first_self_stone].contains(&Stone::NONE) {
                continue;
            }

            let mut cur_row = row;
            let mut cur_column = column;

            for _ in 0..first_self_stone {
                cur_row += i.move_row;
                cur_column += i.move_column;

                self.board[cur_row as usize][cur_column as usize] = self_color
            }
        }

        self.board[row as usize][column as usize] = self_color;
        return Ok("Placed".to_string());
    }

    fn square_state(&self, row: i32, column: i32) -> Result<Stone, String> {
        if 0 <= row && row <= 7 && 0 <= column && column <= 7 {
            Ok(self.board[row as usize][column as usize])
        } else {
            Err("OutofRange".to_string())
        }
    }

    pub fn player_can_place(&self, self_color: Stone) -> bool {
        for row in 0..8 {
            for column in 0..8 {
                if self.can_place_square(row, column, self_color) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn is_game_end(&self) -> bool {
        if self.player_can_place(Stone::BLACK) || self.player_can_place(Stone::WHITE) {
            false
        } else {
            true
        }
    }

    pub fn print_board(&self) -> String {
        let mut board = String::from("🟦1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣\n");

        for (row, alphabet) in (0..8).zip("🇦🇧🇨🇩🇪🇫🇬🇭".chars()) {
            board += &alphabet.to_string();

            for column in 0..8 {
                match self.board[row][column] {
                    Stone::NONE => {
                        if self.can_place_square(row as i32, column as i32, self.turn_stone) {
                            board += "▫️"
                        } else {
                            board += "◽"
                        }
                    }

                    Stone::WHITE => board += "⚪",
                    Stone::BLACK => board += "🔵",
                }
            }

            board += "\n"
        }

        return board;
    }
}
