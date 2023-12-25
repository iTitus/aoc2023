use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::common::parse_lines;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EdgeInfo {
    source: String,
    targets: Vec<String>,
}

impl FromStr for EdgeInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (source, targets) = s.split_once(':').ok_or(())?;
        Ok(Self {
            source: source.trim().to_string(),
            targets: targets.split_whitespace().map(|s| s.to_string()).collect(),
        })
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<EdgeInfo> {
    parse_lines(input).unwrap()
}

fn build_graph(edge_infos: &[EdgeInfo]) -> FxHashMap<&str, FxHashSet<&str>> {
    let mut graph: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    for e in edge_infos {
        for t in &e.targets {
            graph
                .entry(e.source.as_str())
                .or_default()
                .insert(t.as_str());
            graph
                .entry(t.as_str())
                .or_default()
                .insert(e.source.as_str());
        }
    }

    graph
}

fn find_cut<'a>(
    graph: &FxHashMap<&'a str, FxHashSet<&'a str>>,
    k: usize,
) -> FxHashSet<(&'a str, &'a str)> {
    let mut edge_counter: FxHashMap<(&str, &str), usize> = FxHashMap::default();
    let mut seen_keys = FxHashSet::default();
    for v in graph.keys() {
        seen_keys.insert(*v);
        let reachable = dijkstra_all(v, |&n| graph[n].iter().map(|c| (*c, 1usize)));
        for &target in reachable.keys() {
            if seen_keys.contains(target) {
                continue;
            }

            let mut next = target;
            while let Some(&(parent, _)) = reachable.get(next) {
                let edge = if next < parent {
                    (next, parent)
                } else {
                    (parent, next)
                };
                *edge_counter.entry(edge).or_default() += 1;
                next = parent;
            }
        }
    }

    edge_counter
        .into_iter()
        .map(|(e, c)| (-(c as isize), e))
        .k_smallest(k)
        .map(|(_, e)| e)
        .collect()
}

#[aoc(day25, part1)]
pub fn part1(edges: &[EdgeInfo]) -> usize {
    let mut graph = build_graph(edges);
    let cut = find_cut(&graph, 3);
    for (a, b) in &cut {
        graph.get_mut(a).unwrap().remove(b);
        graph.get_mut(b).unwrap().remove(a);
    }

    let all_vertices: Vec<_> = graph.keys().copied().collect();
    let comps = connected_components(&all_vertices, |v| graph[v].iter().copied());

    comps.iter().map(|comp| comp.len()).product()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 54);
    }
}
