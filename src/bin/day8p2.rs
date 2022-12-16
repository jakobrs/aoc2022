use std::time::Instant;

const WIDTH: usize = 99;

fn position(row: usize, column: usize) -> usize {
    row * (WIDTH + 1) + column
}

fn main() {
    let start = Instant::now();

    let mut stdin = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .into_bytes();

    let after_reading = Instant::now();

    for i in &mut stdin {
        *i = i.wrapping_sub(b'0');
    }

    let mut scores: Vec<u32> = vec![1; (WIDTH + 1) * WIDTH];

    for row in 0..WIDTH {
        let mut distances = [0u8; 10];

        for column in 0..WIDTH {
            let pos = position(row, column);
            let value = stdin[pos] as usize;
            scores[pos] *= distances[value] as u32;

            distances[0..=value].fill(0);
            for i in &mut distances {
                *i += 1;
            }
        }
    }
    for row in 0..WIDTH {
        let mut distances = [0u8; 10];

        for column in (0..WIDTH).rev() {
            let pos = position(row, column);
            let value = stdin[pos] as usize;
            scores[pos] *= distances[value] as u32;

            distances[0..=value].fill(0);
            for i in &mut distances {
                *i += 1;
            }
        }
    }

    for column in 0..WIDTH {
        let mut distances = [0u8; 10];

        for row in 0..WIDTH {
            let pos = position(row, column);
            let value = stdin[pos] as usize;
            scores[pos] *= distances[value] as u32;

            distances[0..=value].fill(0);
            for i in &mut distances {
                *i += 1;
            }
        }
    }
    for column in 0..WIDTH {
        let mut distances = [0u8; 10];

        for row in (0..WIDTH).rev() {
            let pos = position(row, column);
            let value = stdin[pos] as usize;
            scores[pos] *= distances[value] as u32;

            distances[0..=value].fill(0);
            for i in &mut distances {
                *i += 1;
            }
        }
    }

    let max = scores.iter().max().unwrap();

    let finished = Instant::now();

    println!("Read time: {:?}", after_reading - start);
    println!("Calculation time: {:?}", finished - after_reading);
    println!("Answer: {max}");
}
