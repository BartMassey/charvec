use rand::prelude::*;
use rand_pcg::Pcg64Mcg;

fn bench_str(n: usize, m: usize) -> u32 {
    // XXX Initialize with "official" PCG state constant.
    let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);
    let index: String = (0..m).map(|_| rng.gen::<char>()).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.gen_range(0..m);
        let c = index.chars().nth(i).unwrap();
        result ^= c as u32;
    }
    result
}

fn bench_charvec(n: usize, m: usize) -> u32 {
    // XXX Initialize with "official" PCG state constant.
    let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);
    let index: Vec<char> = (0..m).map(|_| rng.gen::<char>()).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.gen_range(0..m);
        let c = index[i];
        result ^= c as u32;
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args[2].parse().unwrap();
    let m = args[3].parse().unwrap();
    match &*args[1] {
        "--str" => {
            println!("{}", bench_str(n, m));
        }
        "--charvec" => {
            println!("{}", bench_charvec(n, m));
        }
        _ => panic!("unknown algorithm"),
    }
}
