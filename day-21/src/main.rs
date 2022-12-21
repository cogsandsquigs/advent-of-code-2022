use advent_utils::{files::read, macros::solution};
use anyhow::Result;
use num::complex::Complex64;
use std::{collections::HashMap, mem::swap};

fn main() -> Result<()> {
    let input = read("day-21/input.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "21", part = "2")]
fn part_2(input: &str) -> f64 {
    let mut monkeys = monkeys(input);
    monkeys.insert("humn".into(), Monkey::Number(Complex64::new(0.0, 1.0)));

    let Monkey::Operation { left, right, .. } = monkeys.get("root").unwrap() else {
        unreachable!("`root` monkey does not exist!");
    };

    let mut left = eval_monkeys(&monkeys, left);
    let mut right = eval_monkeys(&monkeys, right);

    // Always keep the human side on the left
    if left.im == 0.0 {
        swap(&mut left, &mut right);
    }

    ((right.re - left.re) / left.im).round()
}

#[solution(day = "21", part = "1")]
fn part_1(input: &str) -> f64 {
    let monkeys = monkeys(input);

    eval_monkeys(&monkeys, "root").re
}

fn eval_monkeys(monkeys: &HashMap<String, Monkey>, id: &str) -> Complex64 {
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

#[derive(Clone, Debug)]
enum Monkey {
    Number(Complex64),

    Operation {
        left: String,
        right: String,
        operation: Operation,
    },
}

#[derive(Clone, Copy, Debug)]
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
    fn operate(&self, left: Complex64, right: Complex64) -> Complex64 {
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
