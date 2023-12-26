use std::ops::{DivAssign, SubAssign};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::SMatrix;
use num::{One, Signed, Zero};

use crate::common::{parse_lines, parse_vec, Rational128, Vec3i, Vec3r128};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Hailstone {
    pos: Vec3i,
    vel: Vec3i,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once('@').ok_or(())?;
        Ok(Self {
            pos: parse_vec(pos).map_err(|_| ())?,
            vel: parse_vec(vel).map_err(|_| ())?,
        })
    }
}

impl Hailstone {
    fn intersect_xy(&self, other: &Hailstone) -> LineIntersect3d {
        intersect_xyz(
            (
                Vec3r128::new(
                    Rational128::from_integer(self.pos.x as _),
                    Rational128::from_integer(self.pos.y as _),
                    Rational128::zero(),
                ),
                Vec3r128::new(
                    Rational128::from_integer(self.vel.x as _),
                    Rational128::from_integer(self.vel.y as _),
                    Rational128::zero(),
                ),
            ),
            (
                Vec3r128::new(
                    Rational128::from_integer(other.pos.x as _),
                    Rational128::from_integer(other.pos.y as _),
                    Rational128::zero(),
                ),
                Vec3r128::new(
                    Rational128::from_integer(other.vel.x as _),
                    Rational128::from_integer(other.vel.y as _),
                    Rational128::zero(),
                ),
            ),
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LineIntersect3d {
    None,
    Equal,
    Point(Rational128, Rational128, Vec3r128),
}

fn intersect_xyz((p, v): (Vec3r128, Vec3r128), (q, u): (Vec3r128, Vec3r128)) -> LineIntersect3d {
    let a = v;
    let b = -u;
    let c = q - p;
    match (a.is_zero(), b.is_zero()) {
        (true, true) => {
            if c.is_zero() {
                LineIntersect3d::Point(
                    Rational128::from_integer(0),
                    Rational128::from_integer(0),
                    p,
                )
            } else {
                LineIntersect3d::None
            }
        }
        (true, false) => {
            // solve p = q + u*s <=> c = b*s <=> s = c/b (elementwise)
            let s = if b.x.is_zero() {
                if b.y.is_zero() {
                    c.z / b.z
                } else {
                    c.y / b.y
                }
            } else {
                c.x / b.x
            };
            if p == q + u * s {
                LineIntersect3d::Point(Rational128::from_integer(0), s, p)
            } else {
                LineIntersect3d::None
            }
        }
        (false, true) => {
            // solve q = p + v*t <=> c = a*t <=> t = c/a (elementwise)
            let t = if a.x.is_zero() {
                if a.y.is_zero() {
                    c.z / a.z
                } else {
                    c.y / a.y
                }
            } else {
                c.x / a.x
            };
            if q == p + v * t {
                LineIntersect3d::Point(t, Rational128::from_integer(0), p)
            } else {
                LineIntersect3d::None
            }
        }
        (false, false) => {
            if a.cross(&b).is_zero() {
                // v and u are parallel
                // if they have any point in common they are equal
                // solve q = p + v*t <=> c = a*t <=> t = c/a (elementwise)
                let t = if a.x.is_zero() {
                    if a.y.is_zero() {
                        c.z / a.z
                    } else {
                        c.y / a.y
                    }
                } else {
                    c.x / a.x
                };
                return if q == p + v * t {
                    LineIntersect3d::Equal
                } else {
                    LineIntersect3d::None
                };
            }

            // gaussian elimination
            let mut m = SMatrix::<Rational128, 3, 3>::from_columns(&[a, b, c]);
            for j in 0..2 {
                let pivot_i = j;

                // find pivot
                if m[(pivot_i, j)].is_zero() {
                    for i in (pivot_i + 1)..3 {
                        if !m[(i, j)].is_zero() {
                            m.swap_rows(pivot_i, i);
                            break;
                        }
                    }

                    assert!(!m[(pivot_i, j)].is_zero());
                }

                // set pivot to 1
                let x = m[(pivot_i, j)];
                m.row_mut(pivot_i).div_assign(x);

                // make column below pivot zero
                for i in (pivot_i + 1)..3 {
                    let set_to_zero = m[(i, j)];
                    if !set_to_zero.is_zero() {
                        let x = m.row(pivot_i) * set_to_zero;
                        m.row_mut(i).sub_assign(x);
                    }
                }
            }

            for j in (0..2).rev() {
                let pivot_i = j;

                // make column above pivot zero
                for i in 0..pivot_i {
                    let set_to_zero = m[(i, j)];
                    if !set_to_zero.is_zero() {
                        let x = m.row(pivot_i) * set_to_zero;
                        m.row_mut(i).sub_assign(x);
                    }
                }
            }

            if m[(2, 2)].is_zero() {
                let t = m[(0, 2)];
                let s = m[(1, 2)];
                LineIntersect3d::Point(t, s, p + v * t)
            } else {
                LineIntersect3d::None
            }
        }
    }
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
            LineIntersect3d::None => false,
            LineIntersect3d::Equal => true,
            LineIntersect3d::Point(t, s, intersect) => {
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

fn convert(v: &Vec3i) -> Vec3r128 {
    Vec3r128::new(
        Rational128::from_integer(v.x as _),
        Rational128::from_integer(v.y as _),
        Rational128::from_integer(v.z as _),
    )
}

fn convert_xy(v: &Vec3i) -> Vec3r128 {
    Vec3r128::new(
        Rational128::from_integer(v.x as _),
        Rational128::from_integer(v.y as _),
        Rational128::zero(),
    )
}

fn brute_force(hailstones: &[Hailstone]) -> (Vec3r128, Vec3r128) {
    fn intersect_all_xy(hailstones: &[Hailstone], rock_vel: Vec3r128) -> Option<Vec3r128> {
        let mut result = None;
        let h0 = &hailstones[0];
        let p = convert_xy(&h0.pos);
        let v = convert_xy(&h0.vel) - rock_vel;
        for h in &hailstones[1..] {
            let q = convert_xy(&h.pos);
            let u = convert_xy(&h.vel) - rock_vel;
            match intersect_xyz((p, v), (q, u)) {
                LineIntersect3d::None => {
                    return None;
                }
                LineIntersect3d::Equal => {}
                LineIntersect3d::Point(_, _, intersect) => match result {
                    None => {
                        result = Some(intersect);
                    }
                    Some(existing_result) => {
                        if existing_result != intersect {
                            return None;
                        }
                    }
                },
            }
        }

        result
    }

    fn intersect_all(hailstones: &[Hailstone], rock_vel: Vec3r128) -> Option<Vec3r128> {
        let mut result = None;
        let h0 = &hailstones[0];
        let p = convert(&h0.pos);
        let v = convert(&h0.vel) - rock_vel;
        for h in &hailstones[1..] {
            let q = convert(&h.pos);
            let u = convert(&h.vel) - rock_vel;
            match intersect_xyz((p, v), (q, u)) {
                LineIntersect3d::None => {
                    return None;
                }
                LineIntersect3d::Equal => {}
                LineIntersect3d::Point(_, _, intersect) => match result {
                    None => {
                        result = Some(intersect);
                    }
                    Some(existing_result) => {
                        if existing_result != intersect {
                            return None;
                        }
                    }
                },
            }
        }

        result
    }

    fn check_xy(hailstones: &[Hailstone], mut rock_vel: Vec3r128) -> Option<(Vec3r128, Vec3r128)> {
        if let Some(_xy_intersect) = intersect_all_xy(hailstones, rock_vel) {
            for z in 0..=1000 {
                // just assume 1000, no guarantees
                let z = Rational128::from_integer(z);
                rock_vel.z = z;
                if let Some(intersect) = intersect_all(hailstones, rock_vel) {
                    return Some((intersect, rock_vel));
                }

                rock_vel.z = -z;
                if let Some(intersect) = intersect_all(hailstones, rock_vel) {
                    return Some((intersect, rock_vel));
                }
            }
        }

        None
    }

    let mut current = Vec3r128::zero();
    if let Some(intersect) = check_xy(hailstones, current) {
        return intersect;
    }
    for n in 1.. {
        for _ in 0..n {
            current.x += if n % 2 == 0 {
                -Rational128::one()
            } else {
                Rational128::one()
            };
            if let Some(intersect) = check_xy(hailstones, current) {
                return intersect;
            }
        }
        for _ in 0..n {
            current.y += if n % 2 == 0 {
                -Rational128::one()
            } else {
                Rational128::one()
            };
            if let Some(intersect) = check_xy(hailstones, current) {
                return intersect;
            }
        }
    }

    unreachable!();
}

#[aoc(day24, part2)]
pub fn part2(hailstones: &[Hailstone]) -> Rational128 {
    // this routine will find a solution starting from two hail trajectories that form a plane
    // sadly that only applies to the example but not the actual input
    let (r, _w) = hailstones
        .iter()
        .tuple_combinations()
        .find_map(|(a, b)| {
            let pa = convert(&a.pos);
            let va = convert(&a.vel);
            let pb = convert(&b.pos);
            let vb = convert(&b.vel);
            let n = va.cross(&vb);
            let (p0, n) = match intersect_xyz((pa, va), (pb, vb)) {
                LineIntersect3d::Equal => {
                    return None;
                }
                LineIntersect3d::None => {
                    if n.is_zero() {
                        (pa, (pa - pb).cross(&va))
                    } else {
                        return None;
                    }
                }
                LineIntersect3d::Point(_, _, _) => (pa, n),
            };

            // now we have a plane between vectors a.vel and b.vel with normal n
            // the rock velocity must lay in that plane so it can cross both lines

            // find two points on the plane so we can find the rock trajectory
            // that trajectory has to go through these points
            let Some((c, d)) = hailstones
                .iter()
                .filter_map(|h| {
                    let q = convert(&h.pos);
                    let u = convert(&h.vel);
                    let denom = u.dot(&n);
                    if denom.is_zero() {
                        // line does not cross the plane
                        return None;
                    }

                    let s = (p0 - q).dot(&n) / denom;
                    let plane_intersect = q + u * s;
                    Some(plane_intersect)
                })
                .take(2)
                .collect_tuple()
            else {
                return None;
            };

            let r = c;
            let w = d - c;

            // verification
            assert!(!w.is_zero());
            for h in hailstones {
                if intersect_xyz((r, w), (convert(&h.pos), convert(&h.vel)))
                    == LineIntersect3d::None
                {
                    return None;
                }
            }

            Some((r, w))
        })
        .unwrap_or_else(|| brute_force(hailstones));

    r.x + r.y + r.z
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
    fn test_intersect_xyz() {
        let rock_vel = Vec3r128::new(
            Rational128::from_integer(-3),
            Rational128::from_integer(1),
            Rational128::from_integer(2),
        );
        assert_eq!(
            intersect_xyz(
                (
                    Vec3r128::new(
                        Rational128::from_integer(19),
                        Rational128::from_integer(13),
                        Rational128::from_integer(30),
                    ),
                    Vec3r128::new(
                        Rational128::from_integer(-2),
                        Rational128::from_integer(1),
                        Rational128::from_integer(-2),
                    ),
                ),
                (
                    Vec3r128::new(
                        Rational128::from_integer(18),
                        Rational128::from_integer(19),
                        Rational128::from_integer(22),
                    ),
                    Vec3r128::new(
                        Rational128::from_integer(-1),
                        Rational128::from_integer(-1),
                        Rational128::from_integer(-2),
                    )
                ),
            ),
            LineIntersect3d::None
        );
        assert_eq!(
            intersect_xyz(
                (
                    Vec3r128::new(
                        Rational128::from_integer(19),
                        Rational128::from_integer(13),
                        Rational128::from_integer(30),
                    ),
                    Vec3r128::new(
                        Rational128::from_integer(-2),
                        Rational128::from_integer(1),
                        Rational128::from_integer(-2),
                    ) - rock_vel,
                ),
                (
                    Vec3r128::new(
                        Rational128::from_integer(18),
                        Rational128::from_integer(19),
                        Rational128::from_integer(22),
                    ),
                    Vec3r128::new(
                        Rational128::from_integer(-1),
                        Rational128::from_integer(-1),
                        Rational128::from_integer(-2),
                    ) - rock_vel
                ),
            ),
            LineIntersect3d::Point(
                Rational128::from_integer(5),
                Rational128::from_integer(3),
                Vec3r128::new(
                    Rational128::from_integer(24),
                    Rational128::from_integer(13),
                    Rational128::from_integer(10),
                ),
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
        assert_eq!(
            part2(&input_generator(INPUT)),
            Rational128::from_integer(47)
        );
    }
}
