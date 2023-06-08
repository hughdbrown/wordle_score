extern crate wordle_score;

use wordle_score::wordle_score_1::wordle_score;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

fn wordle_score_benchmark01(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 01",
        |b| b.iter(|| {
            wordle_score("arena", "aorta")
        }),
    );
}

fn wordle_score_benchmark02(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 02",
        |b| b.iter(|| {
            wordle_score(&"eerie", &"deter")
        }),
    );
}

fn wordle_score_benchmark03(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 03",
        |b| b.iter(|| {
            wordle_score(&"erase", &"chase")
        }),
    );
}

fn wordle_score_benchmark04(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 04",
        |b| b.iter(|| {
            wordle_score(&"tepee", &"erect")
        }),
    );
}

fn wordle_score_benchmark05(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 05",
        |b| b.iter(|| {
            wordle_score(&"erect", &"tepee")
        }),
    );
}

fn wordle_score_benchmark06(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 06",
        |b| b.iter(|| {
            wordle_score(&"beset", &"tepee")
        }),
    );
}

fn wordle_score_benchmark07(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 07",
        |b| b.iter(|| {
            wordle_score(&"tepee", &"beset")
        }),
    );
}

fn wordle_score_benchmark08(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 08",
        |b| b.iter(|| {
            wordle_score(&"crane", &"nacre")
        }),
    );
}

fn wordle_score_benchmark09(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 09",
        |b| b.iter(|| {
            wordle_score(&"crane", &"crete")
        }),
    );
}

fn wordle_score_benchmark10(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 10",
        |b| b.iter(|| {
            wordle_score(&"crane", &"slate")
        }),
    );
}

fn wordle_score_benchmark11(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 11",
        |b| b.iter(|| {
            wordle_score(&"crane", &"toils")
        }),
    );
}

fn wordle_score_benchmark12(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 12",
        |b| b.iter(|| {
            wordle_score(&"abcde", &"fghia")
        }),
    );
}

fn wordle_score_benchmark13(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 13",
        |b| b.iter(|| {
            wordle_score(&"abcde", &"aghij")
        }),
    );
}

fn wordle_score_benchmark14(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 14",
        |b| b.iter(|| {
            wordle_score(&"abcde", &"fghij")
        }),
    );
}

fn wordle_score_benchmark15(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 15",
        |b| b.iter(|| {
            wordle_score(&"eerie", &"chase")
        }),
    );
}

fn wordle_score_benchmark16(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 16",
        |b| b.iter(|| {
            wordle_score(&"spine", &"pines")
        }),
    );
}

fn wordle_score_benchmark17(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 17",
        |b| b.iter(|| {
            wordle_score(&"eerie", &"sheep")
        }),
    );
}

fn wordle_score_benchmark18(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 18",
        |b| b.iter(|| {
            wordle_score(&"sheep", &"eerie")
        }),
    );
}


criterion_group!(
    benches,
    wordle_score_benchmark01,
    wordle_score_benchmark02,
    wordle_score_benchmark03,
    wordle_score_benchmark04,
    wordle_score_benchmark05,
    wordle_score_benchmark06,
    wordle_score_benchmark07,
    wordle_score_benchmark08,
    wordle_score_benchmark09,
    wordle_score_benchmark10,
    wordle_score_benchmark11,
    wordle_score_benchmark12,
    wordle_score_benchmark13,
    wordle_score_benchmark14,
    wordle_score_benchmark15,
    wordle_score_benchmark16,
    wordle_score_benchmark17,
    wordle_score_benchmark18,
);
criterion_main!(benches);

