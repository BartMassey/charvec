use fastrand::*;

#[derive(Debug, Clone, Copy)]
enum Charset {
    Unicode,
    Ascii,
}

impl Charset {
    fn gen_char(self, rng: &mut FastRand) -> char {
        match self {
            Self::Unicode => rng.random(),
            Self::Ascii => rng.rand_range(0u8 as char, char::try_from(0x100u32).unwrap()),
        }
    }
}

fn bench_string(rng: &mut FastRand, charset: Charset, n: usize, m: usize) -> u32 {
    let index: String = (0..m).map(|_| charset.gen_char(rng)).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.rand_range(0, m);
        let c = index.chars().nth(i).unwrap();
        result ^= c as u32;
    }
    result
}

fn bench_charvec(rng: &mut FastRand, charset: Charset, n: usize, m: usize) -> u32 {
    let index: Vec<char> = (0..m).map(|_| charset.gen_char(rng)).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.rand_range(0, m);
        let c = index[i];
        result ^= c as u32;
    }
    result
}

fn bench_baseline(rng: &mut FastRand, _charset: Charset, n: usize, m: usize) -> u32 {
    let mut result = 0;
    for _ in 0..n {
        let i: usize = rng.rand_range(0, m);
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

    let mut rng = FastRand::default();
    println!("{}", algorithm(&mut rng, charset, n, m));
}
