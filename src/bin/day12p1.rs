#![feature(iter_from_generator)]
#![feature(generators)]

type Node = (usize, usize);

const WIDTH: usize = 84;
const HEIGHT: usize = 41;

fn to_coords(n: usize) -> Node {
    (n / WIDTH, n % WIDTH)
}

fn to_index((r, c): Node) -> usize {
    r * WIDTH + c
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();

    let start_pos = stdin.find('S').unwrap();
    let end_pos = stdin.find('E').unwrap();

    let start = to_coords(start_pos);
    let end = to_coords(end_pos);

    let mut grid = stdin.into_bytes();

    grid[start_pos] = b'a';
    grid[end_pos] = b'z';

    let successors = |&(r, c): &Node| {
        let grid = &grid;

        std::iter::from_generator(move || {
            let elevation = grid[to_index((r, c))] - b'a';

            macro_rules! attempt {
                ($cond:expr, $new:expr) => {
                    if $cond {
                        let elevation1 = grid[to_index($new)] - b'a';

                        if elevation1 <= elevation + 1 {
                            yield $new;
                        }
                    }
                };
            }

            attempt!(r > 0, (r - 1, c));
            attempt!(c > 0, (r, c - 1));
            attempt!(r + 1 < HEIGHT, (r + 1, c));
            attempt!(c + 1 < WIDTH - 1, (r, c + 1));
        })
    };

    if let Some(path) = pathfinding::directed::bfs::bfs(&start, successors, |&node| node == end) {
        println!("{}", path.len() - 1);
    } else {
        println!(":(");
    }
}
