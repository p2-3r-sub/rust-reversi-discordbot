/*
QuizKnockさんの量子五目並べのプログラムです。

【理解不能】何色になるか分からない量子で五目並べやってみた【でも楽しそう】
https://www.youtube.com/watch?v=mitAxA3f4U4
*/

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stone {
    Black90,
    Black70,
    White90,
    White70,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservedStone {
    Black,
    White,
    None,
}

#[derive(Debug)]
pub struct QuantumGomoku {
    pub board: [[Stone; 19]; 19],
    pub current_turn: Stone,
}

impl QuantumGomoku {
    pub fn new() -> Self {
        Self {
            board: [[Stone::None; 19]; 19],
            current_turn: Stone::Black90,
        }
    }

    pub fn switch_turn(&mut self) {
        match self.current_turn {
            Stone::Black90 => self.current_turn = Stone::White90,
            Stone::White90 => self.current_turn = Stone::Black70,
            Stone::Black70 => self.current_turn = Stone::White70,
            Stone::White70 => self.current_turn = Stone::Black90,
            Stone::None => panic!("'None' never comes in here."),
        }
    }

    pub fn do_place(&mut self, row: usize, column: usize) -> Result<String, String> {
        if !(row <= 18 && column <= 18) {
            return Err("OutOfBoardRange".to_string());
        }

        if self.board[row][column] != Stone::None {
            return Err("AlreadyPlace".to_string());
        }

        self.board[row][column] = self.current_turn;
        return Ok("Success".to_string());
    }

    fn create_observed_board(&self) -> [[ObservedStone; 19]; 19] {
        let mut observed = [[ObservedStone::None; 19]; 19];
        let mut rng = rand::thread_rng();

        for (irow, rows) in self.board.iter().enumerate() {
            for (icolumn, elem) in rows.iter().enumerate() {
                if *elem == Stone::None {
                    continue;
                }

                let randfloat: f64 = rng.gen();
                let black_possibility = match elem {
                    Stone::Black90 => 0.9,
                    Stone::Black70 => 0.7,
                    Stone::White70 => 0.3,
                    Stone::White90 => 0.1,
                    Stone::None => panic!("'None' never comes in here."),
                };

                if randfloat < black_possibility {
                    observed[irow][icolumn] = ObservedStone::Black
                } else {
                    observed[irow][icolumn] = ObservedStone::White
                }
            }
        }

        return observed;
    }

    pub fn judge_winner(&self) -> (ObservedStone, [[ObservedStone; 19]; 19]) {
        let observed_board = self.create_observed_board();

        let turn_player = match self.current_turn {
            Stone::Black90 => ObservedStone::Black,
            Stone::Black70 => ObservedStone::Black,
            Stone::White90 => ObservedStone::White,
            Stone::White70 => ObservedStone::White,
            Stone::None => panic!("'None' never comes in here."),
        };

        let is_black_connected = Self::is_mass_connected(observed_board, ObservedStone::Black);
        let is_white_connected = Self::is_mass_connected(observed_board, ObservedStone::White);

        let winner = match (is_black_connected, is_white_connected) {
            (true, false) => ObservedStone::Black,
            (false, true) => ObservedStone::White,
            (false, false) => ObservedStone::None,

            (true, true) => match turn_player {
                ObservedStone::Black => ObservedStone::Black,
                ObservedStone::White => ObservedStone::White,
                ObservedStone::None => panic!("'None' never comes in here."),
            },
        };

        return (winner, observed_board);
    }

    fn get_mass_around(
        observed_board: &[[ObservedStone; 19]; 19],
        row: i32,
        column: i32,
    ) -> [[ObservedStone; 5]; 4] {
        let horizontal_line = [ObservedStone::None; 5];
        let vertical_line = [ObservedStone::None; 5];
        let upperright_line = [ObservedStone::None; 5];
        let upperleft_line = [ObservedStone::None; 5];

        let mut lines = [
            horizontal_line,
            vertical_line,
            upperright_line,
            upperleft_line,
        ];

        let horizontal_move = (0, 1);
        let vertical_move = (1, 0);
        let upperright_move = (1, 1);
        let upperleft_move = (1, -1);

        let moves = [
            horizontal_move,
            vertical_move,
            upperright_move,
            upperleft_move,
        ];

        for (line, move_) in lines.iter_mut().zip(moves) {
            let (move_row, move_column) = move_;
            let (cur_row, cur_column) = (row - (move_row * 2), column - (move_column * 2));

            for i in 0..5 {
                let irow = cur_row + (move_row * i);
                let icolumn = cur_column + (move_column * i);

                if 0 <= irow && irow <= 18 && 0 <= icolumn && icolumn <= 18 {
                    line[i as usize] = observed_board[irow as usize][icolumn as usize];
                } else {
                    line[i as usize] = ObservedStone::None;
                }
            }
        }

        return lines;
    }

    fn is_mass_connected(
        observed_board: [[ObservedStone; 19]; 19],
        check_color: ObservedStone,
    ) -> bool {
        let mut is_complete = false;

        for row in 0..19 {
            for column in 0..19 {
                let around_mass = Self::get_mass_around(&observed_board, row, column);
                for line in around_mass {
                    for stone in line {
                        is_complete = true;

                        if stone != check_color {
                            is_complete = false;
                            break;
                        }
                    }

                    if is_complete {
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_change() {
        let mut gomoku = QuantumGomoku::new();
        assert_eq!(gomoku.current_turn, Stone::Black90);

        gomoku.switch_turn();
        assert_eq!(gomoku.current_turn, Stone::White90);

        gomoku.switch_turn();
        assert_eq!(gomoku.current_turn, Stone::Black70);

        gomoku.switch_turn();
        assert_eq!(gomoku.current_turn, Stone::White70);

        gomoku.switch_turn();
        assert_eq!(gomoku.current_turn, Stone::Black90);
    }

    #[test]
    fn do_place() {
        let mut gomoku = QuantumGomoku::new();
        gomoku.board[0][0] = Stone::Black90;

        assert_eq!(Err("OutOfBoardRange".to_string()), gomoku.do_place(19, 19));
        assert_eq!(Err("AlreadyPlace".to_string()), gomoku.do_place(0, 0));
        assert_eq!(Ok("Success".to_string()), gomoku.do_place(0, 1));
    }

    #[test]
    #[ignore]
    fn observe() {
        let mut gomoku = QuantumGomoku::new();

        for i in 0..19 {
            for j in 0..19 {
                gomoku.board[i][j] = Stone::None
            }
        }

        let observed = gomoku.create_observed_board();
        println!("{:#?}", observed)
    }

    #[test]
    fn get_mass_around() {
        let mut observed = [[ObservedStone::Black; 19]; 19];
        observed[0] = [ObservedStone::White; 19];

        assert_eq!(
            [
                [
                    ObservedStone::None,
                    ObservedStone::None,
                    ObservedStone::White,
                    ObservedStone::White,
                    ObservedStone::White,
                ],
                [
                    ObservedStone::None,
                    ObservedStone::None,
                    ObservedStone::White,
                    ObservedStone::Black,
                    ObservedStone::Black,
                ],
                [
                    ObservedStone::None,
                    ObservedStone::None,
                    ObservedStone::White,
                    ObservedStone::Black,
                    ObservedStone::Black,
                ],
                [
                    ObservedStone::None,
                    ObservedStone::None,
                    ObservedStone::White,
                    ObservedStone::None,
                    ObservedStone::None,
                ],
            ],
            QuantumGomoku::get_mass_around(&observed, 0, 0)
        );
    }

    #[test]
    fn is_mass_connected() {
        let mut observed = [[ObservedStone::Black; 19]; 19];
        observed[0] = [ObservedStone::White; 19];

        assert_eq!(
            true,
            QuantumGomoku::is_mass_connected(observed, ObservedStone::Black)
        );

        assert_eq!(
            true,
            QuantumGomoku::is_mass_connected(observed, ObservedStone::White)
        );

        let observed = [[ObservedStone::Black; 19]; 19];

        assert_eq!(
            true,
            QuantumGomoku::is_mass_connected(observed, ObservedStone::Black)
        );

        assert_eq!(
            false,
            QuantumGomoku::is_mass_connected(observed, ObservedStone::White)
        );

        let observed = [[ObservedStone::None; 19]; 19];

        assert_eq!(
            false,
            QuantumGomoku::is_mass_connected(observed, ObservedStone::Black)
        );

        assert_eq!(
            false,
            QuantumGomoku::is_mass_connected(observed, ObservedStone::White)
        );
    }
}
