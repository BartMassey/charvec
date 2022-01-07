use fastrand::Rng;

#[derive(Debug, Clone, Copy)]
enum Charset {
    Unicode,
    Ascii,
}

fn char(rng: &mut Rng) -> char {
    let largest = char::MAX as u32;
    let surrogate_start = 0xd800u32;
    let surrogate_len = 0x800u32;
    let mut val = rng.u32(..largest - surrogate_len);
    if surrogate_start <= val {
        val += surrogate_len;
    }
    val.try_into().unwrap()
}

impl Charset {
    fn gen_char(self, rng: &mut Rng) -> char {
        match self {
            Self::Unicode => char(rng),
            Self::Ascii => rng.u8(..) as char,
        }
    }
}

fn bench_string(rng: &mut Rng, charset: Charset, n: usize, m: usize) -> u32 {
    let index: String = (0..m).map(|_| charset.gen_char(rng)).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.usize(..m);
        let c = index.chars().nth(i).unwrap();
        result ^= c as u32;
    }
    result
}

fn bench_charvec(rng: &mut Rng, charset: Charset, n: usize, m: usize) -> u32 {
    let index: Vec<char> = (0..m).map(|_| charset.gen_char(rng)).collect();
    let mut result = 0;
    for _ in 0..n {
        let i = rng.usize(..m);
        let c = index[i];
        result ^= c as u32;
    }
    result
}

fn bench_baseline(rng: &mut Rng, _charset: Charset, n: usize, m: usize) -> u32 {
    let mut result = 0;
    for _ in 0..n {
        let i: usize = rng.usize(..m);
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

    let mut rng = Rng::new();
    rng.seed(0x123456789abcdef0);
    println!("{}", algorithm(&mut rng, charset, n, m));
}
