use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use nalgebra::Vector2;

pub type Vec2i = Vector2<i64>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'N' | 'U' => Self::North,
            'S' | 'D' => Self::South,
            'E' | 'R' => Self::East,
            'W' | 'L' => Self::West,
            _ => {
                return Err(());
            }
        })
    }
}

impl Direction {
    pub const VALUES: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn vec(&self) -> Vec2i {
        match self {
            Direction::North => Vec2i::new(0, -1),
            Direction::South => Vec2i::new(0, 1),
            Direction::East => Vec2i::new(1, 0),
            Direction::West => Vec2i::new(-1, 0),
        }
    }

    pub fn offset(&self, pos: &Vec2i) -> Vec2i {
        self.offset_with_amount(pos, 1)
    }

    pub fn offset_with_amount(&self, pos: &Vec2i, amount: i64) -> Vec2i {
        pos + amount * self.vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub size_x: usize,
    pub size_y: usize,
    grid: Vec<T>,
}

impl<T> FromStr for Grid<T>
where
    char: TryInto<T>,
{
    type Err = <char as TryInto<T>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut size_x = None;
        let mut size_y = 0;
        let grid = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .flat_map(|l| {
                size_y += 1;
                match size_x {
                    None => size_x = Some(l.len() as _),
                    Some(size_x) if size_x == l.len() as _ => {}
                    _ => {
                        panic!("non rectangular grid");
                    }
                }
                l.chars().map(char::try_into)
            })
            .collect::<Result<_, _>>()?;
        Ok(Grid {
            size_x: size_x.unwrap(),
            size_y,
            grid,
        })
    }
}

impl<T> Grid<T> {
    pub fn in_bounds(&self, pos: &Vec2i) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.size_x && pos.y >= 0 && (pos.y as usize) < self.size_y
    }

    pub fn mod_get(&self, pos: &Vec2i) -> &T {
        let x = pos.x.rem_euclid(self.size_x as i64);
        let y = pos.y.rem_euclid(self.size_y as i64);
        &self[Vec2i::new(x, y)]
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn pos_iter(&self) -> impl Iterator<Item = (Vec2i, &T)> {
        self.grid.iter().enumerate().map(|(i, t)| {
            (
                Vec2i::new((i % self.size_x) as _, (i / self.size_x) as _),
                t,
            )
        })
    }
}

impl<T> Index<Vec2i> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2i) -> &Self::Output {
        &self.grid[(index.x as usize) + self.size_x * (index.y as usize)]
    }
}

impl<T> IndexMut<Vec2i> for Grid<T> {
    fn index_mut(&mut self, index: Vec2i) -> &mut Self::Output {
        &mut self.grid[(index.x as usize) + self.size_x * (index.y as usize)]
    }
}
