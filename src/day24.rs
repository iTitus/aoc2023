use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::{Rational64, Signed};

use crate::common::{parse_lines, Rational128, Vec2i, Vec2r128, Vec3i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Hailstone {
    pos: Vec3i,
    vel: Vec3i,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_vec3i(s: &str) -> Result<Vec3i, ()> {
            let (x, y, z) = s.split(',').map(str::trim).collect_tuple().ok_or(())?;
            Ok(Vec3i::new(
                x.parse().map_err(|_| ())?,
                y.parse().map_err(|_| ())?,
                z.parse().map_err(|_| ())?,
            ))
        }

        let (min, max) = s.split_once('@').ok_or(())?;
        Ok(Self {
            pos: parse_vec3i(min)?,
            vel: parse_vec3i(max)?,
        })
    }
}

impl Hailstone {
    fn intersect_xy(&self, other: &Hailstone) -> LineIntersect2d {
        intersect_xy(
            (self.pos.xy(), self.vel.xy()),
            (other.pos.xy(), other.vel.xy()),
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LineIntersect2d {
    Parallel,
    Equal,
    Point(Rational64, Rational64, Vec2r128),
}

fn intersect_xy((p, v): (Vec2i, Vec2i), (q, u): (Vec2i, Vec2i)) -> LineIntersect2d {
    let a1 = v.x;
    let b1 = -u.x;
    let c1 = q.x - p.x;

    let a2 = v.y;
    let b2 = -u.y;
    let c2 = q.y - p.y;

    let d = a1 * b2 - b1 * a2;
    if d == 0 {
        return if Rational64::new(c1, a1) == Rational64::new(c2, a2) {
            LineIntersect2d::Equal
        } else {
            LineIntersect2d::Parallel
        };
    }

    let t = Rational64::new(c1 * b2 - b1 * c2, d);
    let s = Rational64::new(a1 * c2 - c1 * a2, d);
    let intersect = Vec2r128::new(
        Rational128::from_integer(p.x as _),
        Rational128::from_integer(p.y as _),
    ) + Vec2r128::new(
        Rational128::from_integer(v.x as _),
        Rational128::from_integer(v.y as _),
    ) * Rational128::new(*t.numer() as _, *t.denom() as _);
    LineIntersect2d::Point(t, s, intersect)
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Hailstone> {
    parse_lines(input).unwrap()
}

fn solve1(hailstones: &[Hailstone], min: i64, max: i64) -> usize {
    let min = Rational128::from_integer(min as _);
    let max = Rational128::from_integer(max as _);
    hailstones
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.intersect_xy(b))
        .filter(|i| match i {
            LineIntersect2d::Parallel => false,
            LineIntersect2d::Equal => true,
            LineIntersect2d::Point(t, s, intersect) => {
                !t.is_negative()
                    && !s.is_negative()
                    && (min..=max).contains(&intersect.x)
                    && (min..=max).contains(&intersect.y)
            }
        })
        .count()
}

#[aoc(day24, part1)]
pub fn part1(hailstones: &[Hailstone]) -> usize {
    solve1(hailstones, 200000000000000, 400000000000000)
}

#[aoc(day24, part2)]
pub fn part2(_hailstones: &[Hailstone]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn test_intersect() {
        assert_eq!(
            intersect_xy(
                (Vec2i::new(0, 0), Vec2i::new(1, 1)),
                (Vec2i::new(3, 3), Vec2i::new(2, 2))
            ),
            LineIntersect2d::Equal
        );
        assert_eq!(
            intersect_xy(
                (Vec2i::new(0, 0), Vec2i::new(1, 1)),
                (Vec2i::new(1, 2), Vec2i::new(2, 2))
            ),
            LineIntersect2d::Parallel
        );
        assert_eq!(
            intersect_xy(
                (Vec2i::new(19, 13), Vec2i::new(-2, 1)),
                (Vec2i::new(18, 19), Vec2i::new(-1, -1))
            ),
            LineIntersect2d::Point(
                Rational64::new(7, 3),
                Rational64::new(11, 3),
                Vec2r128::new(Rational128::new(43, 3), Rational128::new(46, 3))
            )
        );
    }

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve1(&input, 7, 27), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 0);
    }
}
