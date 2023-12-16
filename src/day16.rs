use crate::common::{Direction, Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => {
                return Err(());
            }
        })
    }
}

fn simulate(grid: &Grid<Tile>, initial: &(Vec2i, Direction)) -> FxHashSet<Vec2i> {
    let mut visited: FxHashSet<(Vec2i, Direction)> = FxHashSet::default();
    let mut q = vec![*initial];
    while let Some((pos, dir)) = q.pop() {
        if !grid.in_bounds(&pos) || !visited.insert((pos, dir)) {
            continue;
        }

        match grid[pos] {
            Tile::Empty => q.push((dir.offset(&pos), dir)),
            Tile::ForwardMirror => {
                let out_dir = match dir {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                };
                q.push((out_dir.offset(&pos), out_dir));
            }
            Tile::BackwardMirror => {
                let out_dir = match dir {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                };
                q.push((out_dir.offset(&pos), out_dir));
            }
            Tile::VerticalSplitter => match dir {
                Direction::North | Direction::South => {
                    q.push((dir.offset(&pos), dir));
                }
                Direction::East | Direction::West => {
                    q.push((Direction::North.offset(&pos), Direction::North));
                    q.push((Direction::South.offset(&pos), Direction::South));
                }
            },
            Tile::HorizontalSplitter => match dir {
                Direction::North | Direction::South => {
                    q.push((Direction::East.offset(&pos), Direction::East));
                    q.push((Direction::West.offset(&pos), Direction::West));
                }
                Direction::East | Direction::West => {
                    q.push((dir.offset(&pos), dir));
                }
            },
        }
    }

    // unique().count() from itertools did not work
    visited.iter().map(|(pos, _)| *pos).collect()
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

#[aoc(day16, part1)]
pub fn part1(input: &Grid<Tile>) -> usize {
    simulate(input, &(Vec2i::new(0, 0), Direction::East)).len()
}

#[aoc(day16, part2)]
pub fn part2(input: &Grid<Tile>) -> usize {
    (0..input.size_x)
        .flat_map(|x| {
            [
                (Vec2i::new(x as _, 0), Direction::South),
                (
                    Vec2i::new(x as _, (input.size_y - 1) as _),
                    Direction::North,
                ),
            ]
        })
        .chain((0..input.size_y).flat_map(|y| {
            [
                (Vec2i::new(0, y as _), Direction::East),
                (Vec2i::new((input.size_x - 1) as _, y as _), Direction::West),
            ]
        }))
        .map(|initial| simulate(input, &initial).len())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 51);
    }
}
