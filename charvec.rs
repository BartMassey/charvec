use rand::prelude::*;
use rand_pcg::Pcg64Mcg;

#[derive(Debug, Clone, Copy)]
enum Charset {
    Unicode,
    Ascii,
}

impl Charset {
    fn gen_char(self, rng: &mut Pcg64Mcg) -> char {
        match self {
            Self::Unicode => rng.gen(),
            Self::Ascii => rng.gen_range(0u8 as char..=0xffu8 as char),
        }
    }
}

fn bench_string(rng: &mut Pcg64Mcg, charset: Charset, n: usize, m: usize) -> u32 {
    let index: String = (0..m).map(|_| charset.gen_char(rng)).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.gen_range(0..m);
        let c = index.chars().nth(i).unwrap();
        result ^= c as u32;
    }
    result
}

fn bench_charvec(rng: &mut Pcg64Mcg, charset: Charset, n: usize, m: usize) -> u32 {
    let index: Vec<char> = (0..m).map(|_| charset.gen_char(rng)).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.gen_range(0..m);
        let c = index[i];
        result ^= c as u32;
    }
    result
}

fn bench_baseline(rng: &mut Pcg64Mcg, _charset: Charset, n: usize, m: usize) -> u32 {
    let mut result = 0;
    for _ in 0..n {
        let i: usize = rng.gen_range(0..m);
        result ^= i as u32;
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let charset = match args[1].as_str() {
        "--unicode" => Charset::Unicode,
        "--ascii" => Charset::Ascii,
        _ => panic!("unknown charset"),
    };
    let algorithm = match args[2].as_str() {
        "--string" => bench_string,
        "--charvec" => bench_charvec,
        "--baseline" => bench_baseline,
        _ => panic!("unknown algorithm"),
    };
    let n = args[3].parse().unwrap();
    let m = args[4].parse().unwrap();

    // XXX Initialize RNG with "default" PCG state.
    let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);
    println!("{}", algorithm(&mut rng, charset, n, m));
}
