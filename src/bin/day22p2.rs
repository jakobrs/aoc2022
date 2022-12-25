use regex::Regex;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    let grid: Vec<&[u8]> = input.lines().take(200).map(|s| s.as_bytes()).collect();

    let commands = input.lines().skip(4 * 50 + 1).next().unwrap();

    let mut pos = (0isize, 0isize);
    let mut face = 0;
    let mut facing = 0;

    /*
    +--------+    +----+
    |       00001111   |
    |       00001111   |
    | +-----00001111-+ |
    | |     00001111 | |
    | |     2222 ┊   | |
    | |     2222-+   | |
    | |   +-2222     | |
    | |   | 2222     | |
    | | 33334444     | |
    | +-33334444-----+ |
    |   33334444       |
    |   33334444       |
    |   5555 |         |
    +---5555-+         |
        5555           |
        5555           |
          +------------+
     */

    // matrix[face][direction] = (other face, relative rotation);
    let matrix = [
        [(1, 0), (2, 0), (3, 2), (5, 1)],
        [(4, 2), (2, 1), (0, 0), (5, 0)],
        [(1, 3), (4, 0), (3, 3), (0, 0)],
        [(4, 0), (5, 0), (0, 2), (2, 1)],
        [(1, 2), (5, 1), (3, 0), (2, 0)],
        [(4, 3), (1, 0), (0, 3), (3, 0)],
    ];

    let face_positions: [(usize, usize); 6] = [
        (50, 0),
        (100, 0),
        (50, 50),
        (0, 100),
        (50, 100),
        (0, 150),
    ];

    let facing_to_direction = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];

    fn rotate(mut pos: (isize, isize), rotation: usize) -> (isize, isize) {
        for _ in 0..rotation {
            (pos.0, pos.1) = (49 - pos.1, pos.0);
        }

        pos
    }

    let get_abs_pos = |(x, y): (isize, isize), face: usize| -> (usize, usize) {
        let (fx, fy) = face_positions[face];
        (fx.checked_add_signed(x).unwrap(), fy.checked_add_signed(y).unwrap())
    };

    let get = |pos: (isize, isize), face: usize| -> u8 {
        let (x, y) = get_abs_pos(pos, face);
        grid[y][x]
    };

    for command in Regex::new(r"\d+|[RL]").unwrap().find_iter(&commands) {
        match command.as_str() {
            "R" => facing = (facing + 1) % 4,
            "L" => facing = (facing + 3) % 4,
            n => {
                let n: usize = n.parse().unwrap();

                for _ in 0..n {
                    // 1. find prospective position (and rotation)
                    // 2. look at prospective position
                    // 3. if it's *NOT* blocked, move

                    let facing_diff = facing_to_direction[facing];
                    let mut new_pos = (pos.0 + facing_diff.0, pos.1 + facing_diff.1);
                    let mut new_face = face;
                    let mut new_facing = facing;

                    if !(0..50).contains(&new_pos.0) | !(0..50).contains(&new_pos.1) {
                        // moving off face
                        new_pos.0 = new_pos.0.rem_euclid(50);
                        new_pos.1 = new_pos.1.rem_euclid(50);
                        new_face = matrix[face][facing].0;
                        let diff_facing = matrix[face][facing].1;

                        new_pos = rotate(new_pos, diff_facing);
                        new_facing = (facing + diff_facing) % 4;
                    }

                    if get(new_pos, new_face) == b'#' {
                        break;
                    } else {
                        pos = new_pos;
                        face = new_face;
                        facing = new_facing;
                    }
                }
            }
        }
    }

    let (x, y) = get_abs_pos(pos, face);
    println!("{}", (y + 1) * 1000 + (x + 1) * 4 + facing);
}
