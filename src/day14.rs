use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tile {
    Empty,
    Obstacle,
    Rock,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            'O' => Self::Rock,
            _ => {
                return Err(());
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Platform {
    grid: Vec<Vec<Tile>>,
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Platform {
            grid: s
                .trim()
                .lines()
                .map(|l| l.chars().map(Tile::try_from).collect::<Result<_, _>>())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Platform {
    fn size_x(&self) -> usize {
        self.grid[0].len()
    }

    fn size_y(&self) -> usize {
        self.grid.len()
    }

    fn tilt(&self, dir: Direction) -> Platform {
        let sx = self.size_x();
        let sy = self.size_y();
        let mut grid: Vec<Vec<Tile>> = self.grid.clone();
        let mut swap = |x, y, dx, dy| {
            let row: &Vec<Tile> = &grid[y];
            if row[x] == Tile::Rock {
                let mut target_x = x;
                let mut target_y = y;
                loop {
                    let nx = target_x as isize + dx;
                    let ny = target_y as isize + dy;
                    if nx < 0 || (nx as usize) >= sx || ny < 0 || (ny as usize) >= sy {
                        break;
                    }

                    let drow: &Vec<Tile> = &grid[ny as usize];
                    if drow[nx as usize] != Tile::Empty {
                        break;
                    }

                    target_x = nx as usize;
                    target_y = ny as usize;
                }

                if target_x != x || target_y != y {
                    let row: &mut Vec<Tile> = &mut grid[y];
                    row[x] = Tile::Empty;
                    let drow: &mut Vec<Tile> = &mut grid[target_y];
                    drow[target_x] = Tile::Rock;
                }
            }
        };

        match dir {
            Direction::North => {
                for y in 1..sy {
                    for x in 0..sx {
                        swap(x, y, 0, -1);
                    }
                }
            }
            Direction::East => {
                for x in (0..sx - 1).rev() {
                    for y in 0..sy {
                        swap(x, y, 1, 0);
                    }
                }
            }
            Direction::South => {
                for y in (0..sy - 1).rev() {
                    for x in 0..sx {
                        swap(x, y, 0, 1);
                    }
                }
            }
            Direction::West => {
                for x in 1..sx {
                    for y in 0..sy {
                        swap(x, y, -1, 0);
                    }
                }
            }
        };

        Platform { grid }
    }

    fn cycle(&self, n: usize) -> Platform {
        let mut current = self.clone();

        let mut cache: FxHashMap<Platform, usize> = HashMap::default();
        let mut i = 0;
        loop {
            if i == n {
                return current;
            }

            let next = current
                .tilt(Direction::North)
                .tilt(Direction::West)
                .tilt(Direction::South)
                .tilt(Direction::East);
            i += 1;

            if let Some(prev_i) = cache.get(&next) {
                let cycle_length = i - prev_i;
                let how_many_cycles = (n - i) / cycle_length;
                i += how_many_cycles * cycle_length;
            } else {
                cache.insert(next.clone(), i);
            }

            current = next;
        }
    }

    fn total_load(&self) -> usize {
        let sy = self.size_y();
        self.grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let load = sy - y;
                load * row.iter().filter(|t| **t == Tile::Rock).count()
            })
            .sum()
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Platform {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
pub fn part1(input: &Platform) -> usize {
    input.tilt(Direction::North).total_load()
}

#[aoc(day14, part2)]
pub fn part2(input: &Platform) -> usize {
    input.cycle(1_000_000_000).total_load()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 64);
    }
}
