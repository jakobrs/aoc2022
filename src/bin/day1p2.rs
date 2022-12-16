use std::io::BufRead;

fn select_n<T: Ord + Copy>(slice: &[T], n: usize) -> impl Iterator<Item = T> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();

    heap.extend(slice.iter().copied().take(n).map(Reverse));

    for &x in slice.iter().skip(n) {
        heap.push(Reverse(x));
        heap.pop();
    }

    heap.into_iter().map(|Reverse(x)| x)
}

fn main() {
    let stdin = std::io::stdin().lock();

    let mut current = 0;
    let mut counts = vec![];

    for line in stdin.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            counts.push(current);
            current = 0;
        } else {
            let addend: u64 = line.parse().unwrap();
            current += addend;
        }
    }

    println!("{}", select_n(&counts, 3).sum::<u64>());
}
