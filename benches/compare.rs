use std::{hint::black_box, thread::sleep, time::Duration};

use divan::Bencher;

pub const SAMPLE_COUNT: u32 = 1000;
pub const SAMPLE_SIZE: u32 = 1000;
pub struct Token(pub usize);
pub const DELAY: Duration = Duration::from_millis(100);

fn main() {
    divan::main()
}

#[divan::bench(sample_size = SAMPLE_SIZE , sample_count = SAMPLE_COUNT)]
fn kanal_bench(bencher: Bencher) {
    let (sender, receiver) = kanal::unbounded_async();
    std::thread::spawn(move || loop {
        let _ = black_box(sender.send(Token(123)));
    });
    sleep(DELAY);
    bencher.bench_local(|| {
        black_box(receiver.try_recv().unwrap());
    });
}

#[divan::bench(sample_size = SAMPLE_SIZE, sample_count = SAMPLE_COUNT)]
fn corssbeam_bench(bencher: Bencher) {
    let (sender, receiver) = crossbeam::channel::unbounded();
    std::thread::spawn(move || loop {
        let _ = black_box(sender.send(Token(123)));
    });
    sleep(DELAY);
    bencher.bench_local(|| {
        let _ = black_box(&receiver.try_recv());
    });
}

#[divan::bench(sample_size = SAMPLE_SIZE, sample_count = SAMPLE_COUNT)]
fn flume_bench(bencher: Bencher) {
    let (sender, receiver) = flume::unbounded();
    std::thread::spawn(move || loop {
        let _ = sender.send(Token(123));
    });
    sleep(DELAY);
    bencher.bench_local(|| {
        let _ = black_box(&receiver.try_recv());
    });
}
