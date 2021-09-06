use rand::Rng;

fn gen_random_char() -> char {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    CHARSET[rng.gen_range(0..(CHARSET.len()))] as char
}

fn gen_random_str(size: usize) -> String {
    let mut result = String::with_capacity(3);
    for _ in 0..size {
        result.push(gen_random_char());
    }
    result
}

fn main() {
    println!("  ABC DEF GHI JKI HLM NOP QRS TVU WXYZ");
    for i in 0..10 {
        println!("{} {} {} {} {} {} {} {} {} {}",
            i, 
            gen_random_str(3), gen_random_str(3), gen_random_str(3),
            gen_random_str(3), gen_random_str(3), gen_random_str(3),
            gen_random_str(3), gen_random_str(3), gen_random_str(3));
    }
}
