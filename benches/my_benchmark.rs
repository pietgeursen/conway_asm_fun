use conway_asm_fun::*;
use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("update board", |b| {
    //    let mut conways = Conway::new();
    //    b.iter(|| conways.next())
    //});
    c.bench_function("next_board", |b|{
        let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];
        board[6][1] = 1;
        b.iter(||{
            let _ = Conway::next_cell_at_index(black_box(&board), black_box(6),black_box(1));
        });
    });
    c.bench_function("next_board_naiive", |b|{
        let mut board = [[0; Conway::BOARD_WIDTH]; Conway::BOARD_WIDTH];
        board[6][1] = 1;
        b.iter(||{
            let _ = Conway::next_cell_at_index_naive(black_box(&board), black_box(6), black_box(1));
        });

    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
