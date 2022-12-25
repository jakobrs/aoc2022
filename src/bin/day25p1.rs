fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    let n: i64 = stdin.lines().map(|s| parse(s)).sum();

    println!("{n}");
    let s = encode(n);
    println!("{}", s.trim_start_matches('0'));
}

fn parse(s: &str) -> i64 {
    s.chars()
        .rev()
        .map(|ch| match ch {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        })
        .enumerate()
        .map(|(i, j)| 5i64.pow(i as u32) * j)
        .sum()
}

fn encode_digit(i: i64) -> char {
    match i {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!(),
    }
}

fn encode(mut n: i64) -> String {
    (0..=32)
        .rev()
        .map(|i| {
            let place_value = 5i64.pow(i);

            let digit = if n <= 0 {
                -((-n * 2 / place_value) + 1) / 2
            } else {
                ((n * 2 / place_value) + 1) / 2
            };
            n -= digit * place_value;

            encode_digit(digit)
        })
        .collect()
}
