extern crate wordle_score;

use wordle_score::wordle_score_2::wordle_score_2;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

fn wordle_score_benchmark01_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 01_2",
        |b| b.iter(|| {
            wordle_score_2("arena", "aorta")
        }),
    );
}

fn wordle_score_benchmark02_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 02_2",
        |b| b.iter(|| {
            wordle_score_2(&"eerie", &"deter")
        }),
    );
}

fn wordle_score_benchmark03_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 03_2",
        |b| b.iter(|| {
            wordle_score_2(&"erase", &"chase")
        }),
    );
}

fn wordle_score_benchmark04_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 04_2",
        |b| b.iter(|| {
            wordle_score_2(&"tepee", &"erect")
        }),
    );
}

fn wordle_score_benchmark05_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 05_2",
        |b| b.iter(|| {
            wordle_score_2(&"erect", &"tepee")
        }),
    );
}

fn wordle_score_benchmark06_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 06_2",
        |b| b.iter(|| {
            wordle_score_2(&"beset", &"tepee")
        }),
    );
}

fn wordle_score_benchmark07_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 07_2",
        |b| b.iter(|| {
            wordle_score_2(&"tepee", &"beset")
        }),
    );
}

fn wordle_score_benchmark08_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 08_2",
        |b| b.iter(|| {
            wordle_score_2(&"crane", &"nacre")
        }),
    );
}

fn wordle_score_benchmark09_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 09_2",
        |b| b.iter(|| {
            wordle_score_2(&"crane", &"crete")
        }),
    );
}

fn wordle_score_benchmark10_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 10_2",
        |b| b.iter(|| {
            wordle_score_2(&"crane", &"slate")
        }),
    );
}

fn wordle_score_benchmark11_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 11_2",
        |b| b.iter(|| {
            wordle_score_2(&"crane", &"toils")
        }),
    );
}

fn wordle_score_benchmark12_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 12_2",
        |b| b.iter(|| {
            wordle_score_2(&"abcde", &"fghia")
        }),
    );
}

fn wordle_score_benchmark13_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 13_2",
        |b| b.iter(|| {
            wordle_score_2(&"abcde", &"aghij")
        }),
    );
}

fn wordle_score_benchmark14_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 14_2",
        |b| b.iter(|| {
            wordle_score_2(&"abcde", &"fghij")
        }),
    );
}

fn wordle_score_benchmark15_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 15_2",
        |b| b.iter(|| {
            wordle_score_2(&"eerie", &"chase")
        }),
    );
}

fn wordle_score_benchmark16_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 16_2",
        |b| b.iter(|| {
            wordle_score_2(&"spine", &"pines")
        }),
    );
}

fn wordle_score_benchmark17_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 17_2",
        |b| b.iter(|| {
            wordle_score_2(&"eerie", &"sheep")
        }),
    );
}

fn wordle_score_benchmark18_2(c: &mut Criterion) {
    c.bench_function(
        "scoring algorithm 18_2",
        |b| b.iter(|| {
            wordle_score_2(&"sheep", &"eerie")
        }),
    );
}


criterion_group!(
    benches,
    wordle_score_benchmark01_2,
    wordle_score_benchmark02_2,
    wordle_score_benchmark03_2,
    wordle_score_benchmark04_2,
    wordle_score_benchmark05_2,
    wordle_score_benchmark06_2,
    wordle_score_benchmark07_2,
    wordle_score_benchmark08_2,
    wordle_score_benchmark09_2,
    wordle_score_benchmark10_2,
    wordle_score_benchmark11_2,
    wordle_score_benchmark12_2,
    wordle_score_benchmark13_2,
    wordle_score_benchmark14_2,
    wordle_score_benchmark15_2,
    wordle_score_benchmark16_2,
    wordle_score_benchmark17_2,
    wordle_score_benchmark18_2,
);
criterion_main!(benches);

