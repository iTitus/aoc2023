use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::common::{Direction, Grid, Vec2i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => {
                return Err(());
            }
        })
    }
}

impl TryFrom<(Direction, Direction)> for Pipe {
    type Error = ();

    fn try_from(value: (Direction, Direction)) -> Result<Self, Self::Error> {
        Ok(match value {
            (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
                Pipe::Vertical
            }
            (Direction::East, Direction::West) | (Direction::West, Direction::East) => {
                Pipe::Horizontal
            }
            (Direction::North, Direction::East) | (Direction::East, Direction::North) => {
                Pipe::NorthEast
            }
            (Direction::North, Direction::West) | (Direction::West, Direction::North) => {
                Pipe::NorthWest
            }
            (Direction::South, Direction::West) | (Direction::West, Direction::South) => {
                Pipe::SouthWest
            }
            (Direction::South, Direction::East) | (Direction::East, Direction::South) => {
                Pipe::SouthEast
            }
            (_, _) => {
                return Err(());
            }
        })
    }
}

impl Pipe {
    fn is_open(&self, dir: &Direction) -> bool {
        match self {
            Pipe::Vertical => matches!(dir, Direction::North | Direction::South),
            Pipe::Horizontal => matches!(dir, Direction::East | Direction::West),
            Pipe::NorthEast => matches!(dir, Direction::North | Direction::East),
            Pipe::NorthWest => matches!(dir, Direction::North | Direction::West),
            Pipe::SouthWest => matches!(dir, Direction::South | Direction::West),
            Pipe::SouthEast => matches!(dir, Direction::South | Direction::East),
            Pipe::Ground | Pipe::Start => false,
        }
    }
}

fn replace_start(pipes: &mut Grid<Pipe>) -> Vec2i {
    let start = pipes
        .pos_iter()
        .find(|(_, pipe)| **pipe == Pipe::Start)
        .unwrap()
        .0;
    let dirs: (Direction, Direction) = Direction::VALUES
        .iter()
        .filter(|dir| {
            let offset_pos = dir.offset(&start);
            pipes.in_bounds(&offset_pos) && pipes[offset_pos].is_open(&dir.opposite())
        })
        .copied()
        .collect_tuple()
        .unwrap();
    pipes[start] = Pipe::try_from(dirs).unwrap();
    start
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> (Vec2i, Grid<Pipe>) {
    let mut pipes = input.parse().unwrap();
    let start = replace_start(&mut pipes);
    (start, pipes)
}

fn find_cycle(start: &Vec2i, pipes: &Grid<Pipe>) -> Vec<Vec2i> {
    let mut cycle = vec![*start];
    let mut came_from = Direction::North;
    loop {
        let pos = *cycle.last().unwrap();
        let p = &pipes[pos];
        let dir = Direction::VALUES
            .iter()
            .filter(|d| **d != came_from)
            .filter(|d| p.is_open(d))
            .find(|d| {
                let offset_pos = d.offset(&pos);
                pipes.in_bounds(&offset_pos) && pipes[offset_pos].is_open(&d.opposite())
            })
            .unwrap();

        let target_pos = dir.offset(&pos);
        if target_pos == *start {
            break;
        }

        cycle.push(target_pos);
        came_from = dir.opposite();
    }

    cycle
}

#[aoc(day10, part1)]
pub fn part1((start, pipes): &(Vec2i, Grid<Pipe>)) -> usize {
    find_cycle(start, pipes).len() / 2
}

#[aoc(day10, part2, area_scan)]
pub fn part2((start, pipes): &(Vec2i, Grid<Pipe>)) -> usize {
    let cycle: FxHashSet<_> = find_cycle(start, pipes).into_iter().collect();
    let mut inside_cycle_count = 0;
    for y in 0..pipes.size_y {
        let mut inside_cycle = false;
        let mut cycle_opener = None;
        for x in 0..pipes.size_x {
            let pos = Vec2i::new(x as _, y as _);
            if cycle.contains(&pos) {
                match pipes[pos] {
                    Pipe::Horizontal => {}
                    Pipe::Vertical => {
                        inside_cycle = !inside_cycle;
                    }
                    p => {
                        let open_dir = if p.is_open(&Direction::North) {
                            Direction::North
                        } else {
                            Direction::South
                        };
                        match cycle_opener {
                            None => cycle_opener = Some(open_dir),
                            Some(opener_dir) => {
                                cycle_opener = None;
                                if opener_dir != open_dir {
                                    inside_cycle = !inside_cycle;
                                }
                            }
                        }
                    }
                }
            } else if inside_cycle {
                inside_cycle_count += 1;
            }
        }
    }

    inside_cycle_count
}

#[aoc(day10, part2, picks_theorem)]
pub fn part2_pt((start, pipes): &(Vec2i, Grid<Pipe>)) -> usize {
    let cycle = find_cycle(start, pipes);
    // shoelace formula to find the area of the cycle
    let double_area = cycle
        .iter()
        .circular_tuple_windows()
        .map(|(vp, v, vn)| v.x * (vn.y - vp.y))
        .sum::<i64>()
        .unsigned_abs() as usize;
    // pick's theorem to find the number of integer coordinates inside the cycle
    (double_area + 2 - cycle.len()) / 2
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT_1: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;

    const INPUT_2: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    const INPUT_3: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    const INPUT_4: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

    const INPUT_5: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

    const INPUT_6: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    const INPUT_7: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 4);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 4);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(&input_generator(INPUT_3)), 8);
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(part1(&input_generator(INPUT_4)), 8);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&input_generator(INPUT_5)), 4);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator(INPUT_6)), 8);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2(&input_generator(INPUT_7)), 10);
    }

    #[test]
    fn test_part2_pt_1() {
        assert_eq!(part2_pt(&input_generator(INPUT_5)), 4);
    }

    #[test]
    fn test_part2_pt_2() {
        assert_eq!(part2_pt(&input_generator(INPUT_6)), 8);
    }

    #[test]
    fn test_part2_pt_3() {
        assert_eq!(part2_pt(&input_generator(INPUT_7)), 10);
    }
}
