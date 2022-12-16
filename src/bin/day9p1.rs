use std::collections::HashSet;

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn inf_norm(self) -> i32 {
        self.x.abs().max(self.y.abs())
    }
    fn normalise_ish(self: Vec2) -> Vec2 {
        if self.inf_norm() <= 1 {
            Vec2 { x: 0, y: 0 }
        } else {
            Self {
                x: self.x.clamp(-1, 1),
                y: self.y.clamp(-1, 1),
            }
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs
    }
}

impl std::ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn main() -> Result<()> {
    let stdin = std::io::read_to_string(std::io::stdin())?;

    let mut positions = HashSet::new();

    let mut head_position = Vec2 { x: 0, y: 0 };
    let mut tail_position = Vec2 { x: 0, y: 0 };

    for line in stdin.lines() {
        let direction = match line.as_bytes()[0] {
            b'D' => Vec2 { x: 0, y: 1 },
            b'U' => Vec2 { x: 0, y: -1 },
            b'L' => Vec2 { x: -1, y: 0 },
            b'R' => Vec2 { x: 1, y: 0 },
            _ => panic!("gnriueogfiows"),
        };

        for _ in 0..line[2..].parse().unwrap() {
            head_position += direction;
            tail_position += (head_position - tail_position).normalise_ish();

            positions.insert(tail_position);
        }
    }

    println!("{}", positions.len());
    Ok(())
}
