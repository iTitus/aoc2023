use std::hash::BuildHasherDefault;

use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexSet;
use rustc_hash::{FxHashMap, FxHasher};

use crate::common::{Direction, Grid, Vec2i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    fn can_walk_into(&self) -> bool {
        !matches!(self, Self::Forest)
    }

    fn can_walk_out(&self, to: &Direction, ignore_slopes: bool) -> bool {
        match self {
            Self::Path | Self::Forest => true,
            Self::Slope(direction) => ignore_slopes || to == direction,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(Direction::North),
            '>' => Self::Slope(Direction::East),
            'v' => Self::Slope(Direction::South),
            '<' => Self::Slope(Direction::West),
            _ => {
                return Err(());
            }
        })
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

fn longest_path(grid: &Grid<Tile>, ignore_slopes: bool) -> usize {
    fn build_crossing_graph(
        grid: &Grid<Tile>,
        start: Vec2i,
        end: Vec2i,
        ignore_slopes: bool,
    ) -> FxHashMap<Vec2i, Vec<(Vec2i, usize)>> {
        let mut graph: FxHashMap<Vec2i, Vec<(Vec2i, usize)>> = Default::default();
        let mut q = vec![start];
        while let Some(pos) = q.pop() {
            if pos == end || graph.contains_key(&pos) {
                continue;
            }

            let mut children = vec![];
            for initial_dir in Direction::VALUES {
                if !grid[pos].can_walk_out(&initial_dir, ignore_slopes) {
                    continue;
                }

                let mut current = initial_dir.offset(&pos);
                if !grid.in_bounds(&current) {
                    continue;
                }

                let mut came_from = initial_dir.opposite();
                if !grid[current].can_walk_into() {
                    continue;
                }

                let mut length = 1;
                loop {
                    let mut it = Direction::VALUES
                        .iter()
                        .filter(|dir| {
                            **dir != came_from && grid[current].can_walk_out(dir, ignore_slopes)
                        })
                        .map(|dir| (dir.offset(&current), *dir))
                        .filter(|(offset_pos, _)| {
                            grid.in_bounds(offset_pos) && grid[*offset_pos].can_walk_into()
                        });

                    if let Some((neighbor, dir)) = it.next() {
                        if it.next().is_none() {
                            current = neighbor;
                            came_from = dir.opposite();
                            length += 1;
                        } else {
                            // crossing
                            children.push((current, length));
                            q.push(current);
                            break;
                        }
                    } else {
                        // no children
                        if current == end {
                            children.push((current, length));
                            q.push(current);
                        }

                        break;
                    }
                }
            }

            graph.insert(pos, children);
        }
        graph
    }

    let (start, _) = grid
        .pos_iter_row(0)
        .find(|(_, t)| **t == Tile::Path)
        .unwrap();
    let (end, _) = grid
        .pos_iter_row((grid.size_y - 1) as i64)
        .find(|(_, t)| **t == Tile::Path)
        .unwrap();

    // only keep crossings and start+end
    // assumption: |crossings| << |nodes|
    let crossing_graph = build_crossing_graph(grid, start, end, ignore_slopes);

    // possible optimization: use a bitset (u64) for each path to get rid of the indexset

    let mut max_path_length = 0;
    // use indexset: it keeps insertion order and thus remembers our current path
    let mut visited = IndexSet::<_, BuildHasherDefault<FxHasher>>::from_iter([start]);
    let mut q = vec![(crossing_graph[&start].iter(), 0)];
    while let Some((children, path_length)) = q.last_mut() {
        if let Some((child, distance)) = children.next() {
            let new_path_length = *path_length + distance;
            if *child == end {
                max_path_length = max_path_length.max(new_path_length);
            } else if visited.insert(*child) {
                q.push((crossing_graph[child].iter(), new_path_length));
            }
        } else {
            q.pop();
            visited.pop();
        }
    }

    max_path_length
}

#[aoc(day23, part1)]
pub fn part1(grid: &Grid<Tile>) -> usize {
    longest_path(grid, false)
}

#[aoc(day23, part2)]
pub fn part2(grid: &Grid<Tile>) -> usize {
    longest_path(grid, true)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 154);
    }
}
