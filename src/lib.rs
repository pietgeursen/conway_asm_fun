#![feature(portable_simd)]

use rand::random;
use std::{
    arch::aarch64::{int8x8x3_t, vadd_s8, vaddv_s8, vtbl3_s8},
    simd::Simd,
};

pub struct Conway {
    board_1: [[i8; Self::BOARD_WIDTH]; Self::BOARD_WIDTH],
    board_2: [[i8; Self::BOARD_WIDTH]; Self::BOARD_WIDTH],
    is_first_board_active: bool,
}

impl Conway {
    pub const BOARD_WIDTH: usize = 8;
    pub const BOARD_SIZE: usize = Self::BOARD_WIDTH * Self::BOARD_WIDTH;
    //pub const BASE_LUT: Simd<i8, 8> = Simd::from_array([
    //    1,
    //    -1,
    //    -(Self::BOARD_WIDTH as i8),
    //    -1 - Self::BOARD_WIDTH as i8,
    //    1 - Self::BOARD_WIDTH as i8,
    //    Self::BOARD_WIDTH as i8,
    //    -1 + Self::BOARD_WIDTH as i8,
    //    1 + Self::BOARD_WIDTH as i8,
    //]);
    pub const BASE_LUT: Simd<i8, 8> = Simd::from_array([
        0,
        1,
        2,
        Self::BOARD_WIDTH as i8,
        Self::BOARD_WIDTH as i8 + 2,
        Self::BOARD_WIDTH as i8 * 2,
        Self::BOARD_WIDTH as i8 * 2 + 1,
        Self::BOARD_WIDTH as i8 * 2 + 2,
    ]);

    pub fn new() -> Self {
        let mut board_1 = [[0; Self::BOARD_WIDTH]; Self::BOARD_WIDTH];

        board_1.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                *cell = if random::<f32>() > 0.3 { 1 } else { 0 };
            });
        });

        let board_2 = [[0; Self::BOARD_WIDTH]; Self::BOARD_WIDTH];
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
            (0..current_board.len()).for_each(|j| {
                next_board[i][j] = Self::next_cell_at_index(current_board, i, j);
            });
        });
    }

    pub fn print(&self) {
        let current_board = if self.is_first_board_active {
            &self.board_1
        } else {
            &self.board_2
        };

        let board_string = current_board
            .iter()
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

    pub fn next_cell_at_index_naive(
        board: &[[i8; Self::BOARD_WIDTH]; Self::BOARD_WIDTH],
        row: usize,
        col: usize,
    ) -> i8 {
        let neighbours = [
            // Top
            board
                .get(row.wrapping_sub(1))
                .and_then(|column| column.get(col.wrapping_sub(1)))
                .map(|cell| *cell)
                .unwrap_or_default(),
            board
                .get(row.wrapping_sub(1))
                .and_then(|column| column.get(col))
                .map(|cell| *cell)
                .unwrap_or_default(),
            board
                .get(row.wrapping_sub(1))
                .and_then(|column| column.get(col + 1))
                .map(|cell| *cell)
                .unwrap_or_default(),
            // Middle
            board
                .get(row)
                .and_then(|column| column.get(col.wrapping_sub(1)))
                .map(|cell| *cell)
                .unwrap_or_default(),
            board
                .get(row)
                .and_then(|column| column.get(col + 1))
                .map(|cell| *cell)
                .unwrap_or_default(),
            // bottom
            board
                .get(row + 1)
                .and_then(|column| column.get(col.wrapping_sub(1)))
                .map(|cell| *cell)
                .unwrap_or_default(),
            board
                .get(row + 1)
                .and_then(|column| column.get(col))
                .map(|cell| *cell)
                .unwrap_or_default(),
            board
                .get(row + 1)
                .and_then(|column| column.get(col + 1))
                .map(|cell| *cell)
                .unwrap_or_default(),
        ];
        let current_value = unsafe { *board.get_unchecked(row).get_unchecked(col) };
        let neighbour_count = neighbours.iter().sum::<i8>();

        if (current_value > 0 && (neighbour_count == 2 || neighbour_count == 3))
            || (current_value == 0 && neighbour_count == 3)
        {
            1
        } else {
            0
        }
    }

    pub fn next_cell_at_index(
        board: &[[i8; Self::BOARD_WIDTH]; Self::BOARD_WIDTH],
        row: usize,
        col: usize,
    ) -> i8 {
        unsafe {
            let neighbours = [
                // Top
                board
                    .get(row.wrapping_sub(1))
                    .and_then(|column| column.get(col.wrapping_sub(1)))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
                board
                    .get(row.wrapping_sub(1))
                    .and_then(|column| column.get(col))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
                board
                    .get(row.wrapping_sub(1))
                    .and_then(|column| column.get(col + 1))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
                // Middle
                board
                    .get(row)
                    .and_then(|column| column.get(col.wrapping_sub(1)))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
                board
                    .get(row)
                    .and_then(|column| column.get(col + 1))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
                // bottom
                board
                    .get(row + 1)
                    .and_then(|column| column.get(col.wrapping_sub(1)))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
                board
                    .get(row + 1)
                    .and_then(|column| column.get(col))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
                board
                    .get(row + 1)
                    .and_then(|column| column.get(col + 1))
                    .map(|cell| *cell)
                    .unwrap_or_default(),
            ];

            let current_value = *board.get_unchecked(row).get_unchecked(col);

            let neighbours = Simd::from_array(neighbours);
            let neighbour_count = vaddv_s8(neighbours.into());

            if (current_value > 0 && (neighbour_count == 2 || neighbour_count == 3))
                || (current_value == 0 && neighbour_count == 3)
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
        let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

        board[0][0] = 1;

        let result = Conway::next_cell_at_index(&board, 0, 0);

        assert_eq!(result, 0);
    }

    #[test]
    fn under_pop_it_dies_2() {
        let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

        board[7][0] = 1;

        let result = Conway::next_cell_at_index(&board, 7, 0);

        assert_eq!(result, 0);
    }
    //#[test]
    //fn under_pop_it_dies_3() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[9] = 1;

    //    let result = Conway::next_cell_at_index(&board, 9);

    //    assert_eq!(result, 0);
    //}
    //#[test]
    //fn under_pop_it_dies_4() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[7] = 1;
    //    board[8] = 1;
    //    board[9] = 1;

    //    let result = Conway::next_cell_at_index(&board, 8);

    //    assert_eq!(result, 0);
    //}

    //#[test]
    //fn over_pop_it_dies_1() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[0] = 1;
    //    board[1] = 1;
    //    board[2] = 1;
    //    board[8] = 1;
    //    board[9] = 1; //cell
    //    board[10] = 1;
    //    board[16] = 1;
    //    board[17] = 1;
    //    board[18] = 1;

    //    let result = Conway::next_cell_at_index(&board, 9);

    //    assert_eq!(result, 0);
    //}
    //#[test]
    //fn over_pop_it_dies_2() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[0] = 1;
    //    board[1] = 1;
    //    board[2] = 1;
    //    board[8] = 1;
    //    board[9] = 1; //cell

    //    let result = Conway::next_cell_at_index(&board, 9);

    //    assert_eq!(result, 0);
    //}

    //#[test]
    //fn it_lives_1() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[9] = 1;
    //    board[8] = 1;
    //    board[10] = 1;

    //    let result = Conway::next_cell_at_index(&board, 9);

    //    assert_eq!(result, 1);
    //}
    //#[test]
    //fn it_resurects_1() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[1] = 1;
    //    board[8] = 1;
    //    board[10] = 1;

    //    let result = Conway::next_cell_at_index(&board, 9);

    //    assert_eq!(result, 1);
    //}
    //#[test]
    //fn it_lives_2() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[1] = 1;
    //    board[8] = 1;
    //    board[9] = 1;
    //    board[10] = 1;

    //    let result = Conway::next_cell_at_index(&board, 9);

    //    assert_eq!(result, 1);
    //}
    //#[test]
    //fn it_lives_3() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[14] = 1;
    //    board[7] = 1;
    //    board[23] = 1;

    //    let result = Conway::next_cell_at_index(&board, 14);

    //    assert_eq!(result, 1);
    //}
    //#[test]
    //fn it_ressurects_2() {
    //    let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];

    //    board[15] = 1;
    //    board[7] = 1;
    //    board[23] = 1;

    //    let result = Conway::next_cell_at_index(&board, 14);

    //    assert_eq!(result, 1);
    //}
}
