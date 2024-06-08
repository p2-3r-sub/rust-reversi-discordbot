use std::io::Cursor;

use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, ImageFormat, Rgb};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};
use tokio::sync::OnceCell;

use super::gomoku::{ObservedStone, Stone};

const IMAGE_SIZE: u32 = 850;
const BOARD_COLOR: Rgb<u8> = Rgb([216, 179, 77]);

const RECT_WIDTH: u32 = 3;
const RECT_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

static BOARD_TEMPLATE: OnceCell<ImageBuffer<Rgb<u8>, Vec<u8>>> = OnceCell::const_new();

pub struct GenImage {
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl GenImage {
    async fn new() -> Self {
        let self_ = Self {
            img: Self::draw_board_cache().await,
        };

        self_
    }

    fn get_pos(row: i32, column: i32) -> (i32, i32) {
        (75 + (row * 50), 75 + (column * 50))
    }

    fn draw_board() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let image_width = IMAGE_SIZE;
        let image_height = IMAGE_SIZE;
        let mut img = ImageBuffer::from_pixel(image_width, image_height, BOARD_COLOR);

        for i in 0..15 {
            let pos = Self::get_pos(i, 0);
            let rect = Rect::at(pos.0, pos.1).of_size(RECT_WIDTH, 700 + RECT_WIDTH);
            draw_filled_rect_mut(&mut img, rect, RECT_COLOR);

            let pos = Self::get_pos(0, i);
            let rect = Rect::at(pos.0, pos.1).of_size(700 + RECT_WIDTH, RECT_WIDTH);
            draw_filled_rect_mut(&mut img, rect, RECT_COLOR);
        }

        let mut cursor = (226, 226);
        for _ in 0..2 {
            for _ in 0..2 {
                draw_filled_circle_mut(&mut img, cursor, 6, RECT_COLOR);
                cursor.0 += 400;
            }
            cursor.0 -= 800;
            cursor.1 += 400;
        }

        let scale = PxScale { x: 37.5, y: 37.5 };

        // fontフォルダに任意のフォント (font.ttf) を用意する
        let font = FontRef::try_from_slice(include_bytes!("../../font/font.ttf")).unwrap();

        for i in 1..=15 {
            draw_text_mut(
                &mut img,
                RECT_COLOR,
                10,
                i * 50 + 5,
                scale,
                &font,
                &i.to_string(),
            );
        }

        for (i, alphabet) in (1..=15).zip("ABCDEFGHIJKLMNO".chars().into_iter()) {
            draw_text_mut(
                &mut img,
                RECT_COLOR,
                i * 50 + 13,
                10,
                scale,
                &font,
                &alphabet.to_string(),
            );
        }

        return img;
    }

    async fn draw_board_cache() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        BOARD_TEMPLATE
            .get_or_init(|| async { Self::draw_board() })
            .await
            .clone()
    }

    fn push_stone(&mut self, row: i32, column: i32, stone: Stone) {
        let pos = Self::get_pos(row, column);
        let color = match stone {
            Stone::Black90 => Rgb([20, 20, 20]),
            Stone::Black70 => Rgb([65, 65, 65]),
            Stone::White90 => Rgb([230, 230, 230]),
            Stone::White70 => Rgb([150, 150, 150]),
            Stone::None => panic!("'None' never comes in here."),
        };

        draw_filled_circle_mut(&mut self.img, pos, 24, color);
    }

    fn push_observed_stone(&mut self, row: i32, column: i32, stone: ObservedStone) {
        let pos = Self::get_pos(row, column);
        let color = match stone {
            ObservedStone::Black => Rgb([20, 20, 20]),
            ObservedStone::White => Rgb([230, 230, 230]),
            ObservedStone::None => panic!("'None' never comes in here."),
        };

        draw_filled_circle_mut(&mut self.img, pos, 24, color);
    }
}

pub async fn gen_quantum_board_image(board: [[Stone; 19]; 19]) -> Vec<u8> {
    let mut img = GenImage::new().await;

    for (row, rows) in board.iter().enumerate() {
        for (column, stone) in rows.iter().enumerate() {
            if *stone != Stone::None {
                img.push_stone(row as i32, column as i32, *stone)
            }
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut bytes);
    img.img.write_to(&mut writer, ImageFormat::Png).unwrap();

    return bytes;
}

pub async fn gen_quantum_observedboard_image(observed_board: [[ObservedStone; 19]; 19]) -> Vec<u8> {
    let mut img = GenImage::new().await;

    for (row, rows) in observed_board.iter().enumerate() {
        for (column, observed_stone) in rows.iter().enumerate() {
            if *observed_stone != ObservedStone::None {
                img.push_observed_stone(row as i32, column as i32, *observed_stone)
            }
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut bytes);
    img.img.write_to(&mut writer, ImageFormat::Png).unwrap();

    return bytes;
}

#[cfg(test)]
mod tests {
    use super::GenImage;
    use crate::quantum_gomoku::gomoku::{ObservedStone, Stone};

    #[tokio::test]
    #[ignore]
    async fn draw_board() {
        let img = GenImage::new().await;

        img.img.save("test.png").expect("error");
    }

    #[tokio::test]
    #[ignore]
    async fn push_stone() {
        let mut img = GenImage::new().await;

        for i in 0..9 {
            img.push_stone(i, i, Stone::Black90);
            img.push_stone(i + 1, i, Stone::Black70);
            img.push_stone(i + 2, i, Stone::White70);
            img.push_stone(i + 3, i, Stone::White90);
        }

        img.img.save("test.png").expect("error");
    }

    #[tokio::test]
    #[ignore]
    async fn push_observed_stone() {
        let mut img = GenImage::new().await;

        for i in 0..9 {
            img.push_observed_stone(i, i, ObservedStone::Black);
            img.push_observed_stone(i + 1, i, ObservedStone::White);
        }

        img.img.save("test.png").expect("error");
    }

    #[tokio::test]
    #[ignore]
    async fn gen_board_image() {
        let mut board = [[Stone::White70; 19]; 19];
        board[0] = [Stone::Black90; 19];

        let vec = super::gen_quantum_board_image(board).await;
        println!("{:?}", vec);
    }

    #[tokio::test]
    #[ignore]
    async fn gen_observedboard_image() {
        let mut board = [[ObservedStone::White; 19]; 19];
        board[0] = [ObservedStone::Black; 19];

        let vec = super::gen_quantum_observedboard_image(board).await;
        println!("{:?}", vec);
    }
}
