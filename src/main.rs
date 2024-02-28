#![feature(portable_simd)]

use std::{
    arch::{
        aarch64::{int8x8x3_t, vaddv_s16, vaddv_s8, vaddvq_s16, vaddvq_s8, vtbl1_s8, vtbl3_s8},
        asm,
    },
    simd::Simd,
};

fn main() {
    // Multiply x by 6 using shifts and adds
    let mut x: u64 = 4;
    unsafe {
        asm!(
            "add {x}, {x}, {x} ",
            x = inout(reg) x,
        );
    }
    assert_eq!(x, 4 + 4);

    let width = 8i8;
    let mut board = [0i8; 64];

    board.iter_mut().enumerate().for_each(|(i, b)| *b = i as i8);

    // Just for an example value
    let index = 5;

    let lut: [i8; 8] = [
        index + 1,
        index - 1,
        index - width,
        index - 1 - width,
        index + 1 - width,
        index + width,
        index - 1 + width,
        index + 1 + width,
    ];
    let lut = Simd::from_array(lut);
    //let board = Simd::from_slice(&board[.. (width as usize * 3)]);

    unsafe {
        let top_row = Simd::from_slice(&board[0..8]);
        let middle_row = Simd::from_slice(&board[8..16]);
        let bottom_row = Simd::from_slice(&board[16..24]);

        println!("{:?}", top_row);
        println!("{:?}", middle_row);
        println!("{:?}", bottom_row);

        let board_rows = int8x8x3_t(top_row.into(), middle_row.into(), bottom_row.into());
        let neighbours = vtbl3_s8(board_rows, lut.into());
        println!("neighbours: {:?}", neighbours);
        let y = vaddv_s8(neighbours);
        println!("y: {:?}", y);
    }
}
