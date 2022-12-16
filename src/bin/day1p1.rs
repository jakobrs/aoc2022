use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();

    let mut line = String::new();

    let mut max = 0;
    let mut current = 0;

    loop {
        line.clear();

        match stdin.read_line(&mut line).unwrap() {
            0 => break,
            1 => {
                if current > max {
                    max = current;
                }
                current = 0;
            }
            _ => {
                let addend: u64 = line.trim().parse().unwrap();
                current += addend;
            }
        }
    }

    println!("{max}");
}
