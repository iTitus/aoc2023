use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuleTarget {
    Reject,
    Accept,
    Workflow(String),
}

impl From<&str> for RuleTarget {
    fn from(s: &str) -> Self {
        match s.trim() {
            "R" => Self::Reject,
            "A" => Self::Accept,
            _ => Self::Workflow(s.to_string()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConditionVariable {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for ConditionVariable {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => {
                return Err(());
            }
        })
    }
}

impl ConditionVariable {
    fn get(&self, part: &Part) -> i64 {
        match self {
            Self::X => part.x,
            Self::M => part.m,
            Self::A => part.a,
            Self::S => part.s,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConditionOperation {
    LessThan,
    GreaterThan,
}

impl TryFrom<char> for ConditionOperation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '<' => Self::LessThan,
            '>' => Self::GreaterThan,
            _ => {
                return Err(());
            }
        })
    }
}

impl ConditionOperation {
    fn matches(&self, a: i64, b: i64) -> bool {
        match self {
            Self::LessThan => a < b,
            Self::GreaterThan => a > b,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RuleCondition {
    variable: ConditionVariable,
    operation: ConditionOperation,
    number: i64,
}

impl FromStr for RuleCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let variable = ConditionVariable::try_from(it.next().ok_or(())?)?;
        let operation = ConditionOperation::try_from(it.next().ok_or(())?)?;
        let number = s[2..].parse().map_err(|_| ())?;
        Ok(Self {
            variable,
            operation,
            number,
        })
    }
}

impl RuleCondition {
    fn matches(&self, part: &Part) -> bool {
        self.operation.matches(self.variable.get(part), self.number)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rule {
    condition: Option<RuleCondition>,
    target: RuleTarget,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some((condition, target)) = s.split_once(':') {
            Self {
                condition: Some(condition.trim().parse()?),
                target: RuleTarget::from(target.trim()),
            }
        } else {
            Self {
                condition: None,
                target: RuleTarget::from(s.trim()),
            }
        })
    }
}

impl Rule {
    fn apply_to(&self, part: &Part) -> Option<&RuleTarget> {
        if self.condition.is_none() || self.condition.unwrap().matches(part) {
            Some(&self.target)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Workflow {
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rules: s
                .trim_matches(|c: char| c == '{' || c == '}' || c.is_whitespace())
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Workflow {
    fn apply_to(&self, part: &Part) -> &RuleTarget {
        self.rules
            .iter()
            .filter_map(|r| r.apply_to(part))
            .next()
            .unwrap()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Workflows {
    workflows: FxHashMap<String, Workflow>,
}

impl FromStr for Workflows {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            workflows: s
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(|l| {
                    let i = l.find('{').ok_or(())?;
                    Ok((l[..i].to_string(), l[i..].parse()?))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Workflows {
    fn accept(&self, part: &Part) -> bool {
        let mut current = "in";
        loop {
            match self.workflows[current].apply_to(part) {
                RuleTarget::Reject => {
                    return false;
                }
                RuleTarget::Accept => {
                    return true;
                }
                RuleTarget::Workflow(next) => {
                    current = next.as_str();
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;
        for value in value
            .trim_matches(|c: char| c == '{' || c == '}' || c.is_whitespace())
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let (var, val) = value.split_once('=').ok_or(())?;
            let val = val.parse().map_err(|_| ())?;
            if var.len() != 1 {
                return Err(());
            }
            let var = match ConditionVariable::try_from(var.chars().next().unwrap())? {
                ConditionVariable::X => &mut x,
                ConditionVariable::M => &mut m,
                ConditionVariable::A => &mut a,
                ConditionVariable::S => &mut s,
            };
            if var.is_some() {
                return Err(());
            }
            *var = Some(val);
        }
        Ok(Self {
            x: x.ok_or(())?,
            m: m.ok_or(())?,
            a: a.ok_or(())?,
            s: s.ok_or(())?,
        })
    }
}

impl Part {
    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> (Workflows, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    (
        workflows.parse().unwrap(),
        parts
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap(),
    )
}

#[aoc(day19, part1)]
pub fn part1((workflows, parts): &(Workflows, Vec<Part>)) -> i64 {
    parts
        .iter()
        .filter(|p| workflows.accept(p))
        .map(Part::rating)
        .sum()
}

#[aoc(day19, part2)]
pub fn part2((workflows, _): &(Workflows, Vec<Part>)) -> i64 {
    #[derive(Debug, Copy, Clone)]
    struct Ranges {
        x: (i64, i64),
        m: (i64, i64),
        a: (i64, i64),
        s: (i64, i64),
    }

    impl Ranges {
        fn split(&self, condition: &Option<RuleCondition>) -> (Ranges, Ranges) {
            match condition {
                None => (
                    *self,
                    Self {
                        x: (0, 0),
                        m: (0, 0),
                        a: (0, 0),
                        s: (0, 0),
                    },
                ),
                Some(condition) => {
                    let mut a = *self;
                    let mut b = *self;
                    let (a_var, b_var) = match condition.variable {
                        ConditionVariable::X => (&mut a.x, &mut b.x),
                        ConditionVariable::M => (&mut a.m, &mut b.m),
                        ConditionVariable::A => (&mut a.a, &mut b.a),
                        ConditionVariable::S => (&mut a.s, &mut b.s),
                    };

                    let n = condition.number;
                    let min = a_var.0;
                    let max = a_var.1 + 1;
                    match condition.operation {
                        ConditionOperation::LessThan => {
                            a_var.1 = n.clamp(min, max);
                            b_var.0 = n.clamp(min, max);
                        }
                        ConditionOperation::GreaterThan => {
                            a_var.0 = (n + 1).clamp(min, max);
                            b_var.1 = (n + 1).clamp(min, max);
                        }
                    }

                    (a, b)
                }
            }
        }

        fn volume(&self) -> i64 {
            (self.x.1 - self.x.0)
                * (self.m.1 - self.m.0)
                * (self.a.1 - self.a.0)
                * (self.s.1 - self.s.0)
        }
    }

    let mut accepted = 0;
    let mut q = vec![(
        "in",
        Ranges {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        },
    )];
    'outer: while let Some((name, ranges)) = q.pop() {
        if ranges.volume() == 0 {
            continue;
        }

        let workflow = &workflows.workflows[name];
        let mut current_ranges = ranges;
        for rule in &workflow.rules {
            let (a, b) = current_ranges.split(&rule.condition);
            current_ranges = b;
            match &rule.target {
                RuleTarget::Reject => {}
                RuleTarget::Accept => accepted += a.volume(),
                RuleTarget::Workflow(name) => q.push((name.as_str(), a)),
            }

            if current_ranges.volume() == 0 {
                continue 'outer;
            }
        }

        unreachable!();
    }

    accepted
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 167409079868000);
    }
}
