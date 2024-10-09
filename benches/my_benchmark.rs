use conway_asm_fun::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("update board", |b| {
    //    let mut conways = Conway::new();
    //    b.iter(|| conways.next())
    //});
    c.bench_function("next_board", |b|{
        let mut board = [0i8; Conway::BOARD_SIZE];
        board[10] = 1;
        b.iter(||{
            let _ = Conway::next_cell_at_index(&board, 9);
        });
    });
    c.bench_function("next_board_naiive", |b|{
        let mut board = [0i8; Conway::BOARD_SIZE];
        board[10] = 1;
        b.iter(||{
            let _ = Conway::next_cell_at_index_naive(&board, 9);
        });

    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
