use rand::prelude::*;
use rand_pcg::Pcg64Mcg;

fn bench_string(rng: &mut Pcg64Mcg, n: usize, m: usize) -> u32 {
    let index: String = (0..m).map(|_| rng.gen::<char>()).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.gen_range(0..m);
        let c = index.chars().nth(i).unwrap();
        result ^= c as u32;
    }
    result
}

fn bench_charvec(rng: &mut Pcg64Mcg, n: usize, m: usize) -> u32 {
    let index: Vec<char> = (0..m).map(|_| rng.gen::<char>()).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.gen_range(0..m);
        let c = index[i];
        result ^= c as u32;
    }
    result
}

fn bench_baseline(rng: &mut Pcg64Mcg, n: usize, m: usize) -> u32 {
    let mut result = 0;
    for _ in 0..n {
        let i: usize = rng.gen_range(0..m);
        result ^= i as u32;
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args[2].parse().unwrap();
    let m = args[3].parse().unwrap();
    let algorithm = match &*args[1] {
        "--string" => bench_string,
        "--charvec" => bench_charvec,
        "--baseline" => bench_baseline,
        _ => panic!("unknown algorithm"),
    };
    // XXX Initialize with "official" PCG state constant.
    let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);
    println!("{}", algorithm(&mut rng, n, m));
}
