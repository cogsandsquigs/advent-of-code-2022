use advent_utils::{files::read, macros::solution};
use anyhow::Result;
use std::{collections::HashMap, mem::swap, vec};

fn main() -> Result<()> {
    let input = read("day-21/input.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "21", part = "2")]
fn part_2(input: &str) -> i64 {
    let monkeys = monkeys(input);

    let Monkey::Operation{ mut left, mut right, .. } = monkeys.get("root").cloned().unwrap() else {
        unreachable!("`root` should be an operation monkey!");
    };

    // Always have the human on the right side
    if contains_human(&monkeys, &left) {
        swap(&mut left, &mut right);
    }

    let mut left = eval_monkeys(&monkeys, &left);
    let ops = reversed_ops(&monkeys, &right);

    for (op, right) in ops {
        left = op.operate(left, right);
    }

    left
}

fn reversed_ops(monkeys: &HashMap<String, Monkey>, id: &str) -> Vec<(Operation, i64)> {
    if id == "humn" {
        vec![]
    } else {
        match monkeys.get(id).unwrap().clone() {
            Monkey::Number(..) => unreachable!("Should not get a number monkey here!"),
            Monkey::Operation {
                mut left,
                mut right,
                operation,
            } => {
                // Always have the human on the right side
                if contains_human(monkeys, &left) {
                    swap(&mut left, &mut right);
                }

                let mut ops = vec![(operation.opposite(), eval_monkeys(monkeys, &left))];
                ops.append(&mut reversed_ops(monkeys, &right));

                ops
            }
        }
    }
}

fn contains_human(monkeys: &HashMap<String, Monkey>, id: &str) -> bool {
    if id == "humn" {
        true
    } else {
        match monkeys.get(id).unwrap() {
            Monkey::Number(..) => false,
            Monkey::Operation { left, right, .. } => {
                contains_human(monkeys, left) || contains_human(monkeys, right)
            }
        }
    }
}

#[solution(day = "21", part = "1")]
fn part_1(input: &str) -> i64 {
    let monkeys = monkeys(input);

    eval_monkeys(&monkeys, "root")
}

fn eval_monkeys(monkeys: &HashMap<String, Monkey>, id: &str) -> i64 {
    let monkey = monkeys.get(id).unwrap();

    match monkey {
        Monkey::Number(v) => *v,
        Monkey::Operation {
            left,
            right,
            operation,
        } => operation.operate(eval_monkeys(monkeys, left), eval_monkeys(monkeys, right)),
    }
}

fn monkeys(input: &str) -> HashMap<String, Monkey> {
    let mut monkey_set = HashMap::new();

    input.lines().for_each(|line| {
        let sep: Vec<&str> = line.split_whitespace().collect();
        let id = sep[0][0..4].to_string();
        let monkey: Monkey;

        if sep.len() == 2 {
            monkey = Monkey::Number(sep[1].parse().unwrap())
        } else {
            monkey = Monkey::Operation {
                left: sep[1].to_string(),
                right: sep[3].to_string(),
                operation: Operation::try_from(sep[2]).unwrap(),
            }
        }

        monkey_set.insert(id, monkey);
    });

    monkey_set
}

#[derive(Clone, Debug, Hash)]
enum Monkey {
    Number(i64),

    Operation {
        left: String,
        right: String,
        operation: Operation,
    },
}

#[derive(Clone, Copy, Debug, Hash)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            x => Err(format!("Unknown operation {x}!")),
        }
    }
}

impl Operation {
    fn operate(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
        }
    }
}
