#![feature(portable_simd)]

use rand::random;
use std::{
    arch::aarch64::{int8x8x3_t, vadd_s8, vaddv_s8, vtbl3_s8},
    simd::Simd,
};

pub struct Conway {
    board_1: [i8; Self::BOARD_WIDTH * Self::BOARD_WIDTH],
    board_2: [i8; Self::BOARD_WIDTH * Self::BOARD_WIDTH],
    is_first_board_active: bool,
}

impl Conway {
    pub const BOARD_WIDTH: usize = 8;
    pub const BOARD_SIZE: usize = Self::BOARD_WIDTH * Self::BOARD_WIDTH;
    // This restricts the max size of the board to 127*127
    pub const BASE_LUT: Simd<i8, 8> = Simd::from_array([
        1,
        -1,
        -(Self::BOARD_WIDTH as i8),
        -1 - Self::BOARD_WIDTH as i8,
        1 - Self::BOARD_WIDTH as i8,
        Self::BOARD_WIDTH as i8,
        -1 + Self::BOARD_WIDTH as i8,
        1 + Self::BOARD_WIDTH as i8,
    ]);

    pub fn new() -> Self {
        let mut board_1 = [0; Self::BOARD_SIZE];

        board_1.iter_mut().for_each(|cell| {
            *cell = if random::<f32>() > 0.3 { 1 } else { 0 };
        });

        let board_2 = [0; Self::BOARD_SIZE];
        let is_first_board_active = true;

        println!("{:?}", Self::BASE_LUT);

        Self {
            board_1,
            board_2,
            is_first_board_active,
        }
    }

    pub fn next(&mut self) {
        let (current_board, next_board) = if self.is_first_board_active {
            (&self.board_1, &mut self.board_2)
        } else {
            (&self.board_2, &mut self.board_1)
        };
        self.is_first_board_active = !self.is_first_board_active;

        (0..current_board.len()).for_each(|i| {
            next_board[i] = Self::next_cell_at_index(current_board, i);
        });
    }

    pub fn print(&self) {
        let current_board = if self.is_first_board_active {
            &self.board_1
        } else {
            &self.board_2
        };

        let board_string = current_board
            .chunks(Self::BOARD_WIDTH)
            .map(|chunk| {
                let mut row_string = chunk
                    .iter()
                    .map(|cell| if *cell > 0 { 'X' } else { '_' })
                    .collect::<String>();
                row_string.push('\n');
                row_string
            })
            .collect::<String>();

        //print!("{}[2J", 27 as char);
        println!("{board_string}");
    }

    pub fn next_cell_at_index_naive(board: &[i8], index: usize) -> i8 {
        let empty_row = [0; 8];
        let middle_row_index = index / Self::BOARD_WIDTH;
        // TODO: top and bottom rows need to be an option or something
        // The bug here is when the middle row is 0 the top row needs to be an empty row
        // Saturating sub ain't it
        let top_row_index = middle_row_index.saturating_sub(1);
        let bottom_row_index = middle_row_index + 1;

        let top_row = board
            .get(0 + (top_row_index * Self::BOARD_WIDTH)..8 + (top_row_index * Self::BOARD_WIDTH))
            .unwrap_or(&empty_row);
        let middle_row = board
            .get(
                0 + (middle_row_index * Self::BOARD_WIDTH)
                    ..8 + (middle_row_index * Self::BOARD_WIDTH),
            )
            .unwrap_or(&empty_row);
        let bottom_row = board
            .get(
                0 + (bottom_row_index * Self::BOARD_WIDTH)
                    ..8 + (bottom_row_index * Self::BOARD_WIDTH),
            )
            .unwrap_or(&empty_row);

        let neighbour_count: i8 = top_row.into_iter().sum::<i8>()
            + middle_row.into_iter().sum::<i8>()
            + bottom_row.into_iter().sum::<i8>()
            - board[index];
        let current_value = unsafe { board.get_unchecked(index) };

        if (*current_value > 0 && (neighbour_count == 2 || neighbour_count == 3))
            || (*current_value == 0 && neighbour_count == 3)
        {
            1
        } else {
            0
        }
    }

    pub fn next_cell_at_index(board: &[i8], index: usize) -> i8 {
        unsafe {
            let empty_row = [0; 8];

            // Creates a vec 8 filled with the index value
            let index_vec = Simd::from_array([index as i8; 8]);
            // Add the index to each value in the base lut
            let lut = vadd_s8(Self::BASE_LUT.into(), index_vec.into());

            let middle_row_index = index / Self::BOARD_WIDTH;

            // TODO: top and bottom rows need to be an option or something
            // The bug here is when the middle row is 0 the top row needs to be an empty row
            // Saturating sub ain't it

            let bottom_row_index = middle_row_index + 1;

            let top_row = if middle_row_index == 0 {
                &empty_row
            } else {
                let top_row_index = middle_row_index - 1;
                board
                    .get(
                        0 + (top_row_index * Self::BOARD_WIDTH)
                            ..8 + (top_row_index * Self::BOARD_WIDTH),
                    )
                    .unwrap_or(&empty_row)
            };

            let middle_row = board
                .get(
                    0 + (middle_row_index * Self::BOARD_WIDTH)
                        ..8 + (middle_row_index * Self::BOARD_WIDTH),
                )
                .unwrap_or(&empty_row);
            let bottom_row = board
                .get(
                    0 + (bottom_row_index * Self::BOARD_WIDTH)
                        ..8 + (bottom_row_index * Self::BOARD_WIDTH),
                )
                .unwrap_or(&empty_row);

            let top_row = Simd::from_slice(top_row);
            let middle_row = Simd::from_slice(middle_row);
            let bottom_row = Simd::from_slice(bottom_row);

            let board_rows = int8x8x3_t(top_row.into(), middle_row.into(), bottom_row.into());
            let neighbours = vtbl3_s8(board_rows, lut.into());

            let neighbour_count = vaddv_s8(neighbours);

            let current_value = board.get_unchecked(index);

            if (*current_value > 0 && (neighbour_count == 2 || neighbour_count == 3))
                || (*current_value == 0 && neighbour_count == 3)
            {
                1
            } else {
                0
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Conway;

    #[test]
    fn under_pop_it_dies_1() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[0] = 1;

        let result = Conway::next_cell_at_index(&board, 0);

        assert_eq!(result, 0);
    }

    #[test]
    fn under_pop_it_dies_2() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[56] = 1;

        let result = Conway::next_cell_at_index(&board, 56);

        assert_eq!(result, 0);
    }
    #[test]
    fn under_pop_it_dies_3() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[9] = 1;

        let result = Conway::next_cell_at_index(&board, 9);

        assert_eq!(result, 0);
    }
    #[test]
    fn under_pop_it_dies_4() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[7] = 1;
        board[8] = 1;
        board[9] = 1;

        let result = Conway::next_cell_at_index(&board, 8);

        assert_eq!(result, 0);
    }


    #[test]
    fn over_pop_it_dies_1() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[0] = 1;
        board[1] = 1;
        board[2] = 1;
        board[8] = 1;
        board[9] = 1; //cell
        board[10] = 1;
        board[16] = 1;
        board[17] = 1;
        board[18] = 1;

        let result = Conway::next_cell_at_index(&board, 9);

        assert_eq!(result, 0);
    }
    #[test]
    fn over_pop_it_dies_2() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[0] = 1;
        board[1] = 1;
        board[2] = 1;
        board[8] = 1;
        board[9] = 1; //cell

        let result = Conway::next_cell_at_index(&board, 9);

        assert_eq!(result, 0);
    }

    #[test]
    fn it_lives_1() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[9] = 1;
        board[8] = 1;
        board[10] = 1;

        let result = Conway::next_cell_at_index(&board, 9);

        assert_eq!(result, 1);
    }
    #[test]
    fn it_resurects_1() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[1] = 1;
        board[8] = 1;
        board[10] = 1;

        let result = Conway::next_cell_at_index(&board, 9);

        assert_eq!(result, 1);
    }
    #[test]
    fn it_lives_2() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[1] = 1;
        board[8] = 1;
        board[9] = 1;
        board[10] = 1;

        let result = Conway::next_cell_at_index(&board, 9);

        assert_eq!(result, 1);
    }
    #[test]
    fn it_lives_3() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[14] = 1;
        board[7] = 1;
        board[23] = 1;

        let result = Conway::next_cell_at_index(&board, 14);

        assert_eq!(result, 1);
    }
    #[test]
    fn it_ressurects_2() {
        let mut board = [0i8; Conway::BOARD_SIZE];

        board[15] = 1;
        board[7] = 1;
        board[23] = 1;

        let result = Conway::next_cell_at_index(&board, 14);

        assert_eq!(result, 1);
    }
}
