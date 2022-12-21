use advent_utils::{files::read, macros::solution};
use anyhow::Result;
use std::{collections::HashMap, mem::swap, vec};

fn main() -> Result<()> {
    let input = read("day-21/input.test.txt")?;

    part_1(&input);

    part_2(&input);

    Ok(())
}

#[solution(day = "21", part = "2")]
fn part_2(input: &str) -> i64 {
    let monkeys = monkeys(input);

    equalize_monkeys(&monkeys)
}

fn equalize_monkeys(monkeys: &HashMap<String, Monkey>) -> i64 {
    let Monkey::Operation{ left: root_left, right: root_right, .. } = monkeys.get("root").unwrap() else {
        unreachable!("`root` should be an operation monkey!");
    };

    let mut left = collect_monkeys(monkeys, root_left);
    let mut right = collect_monkeys(monkeys, root_right);

    // Do this so we know which one has the human: the right one always has the human
    // after this.
    if contains_human(&left) {
        swap(&mut left, &mut right);
    }

    let ops = get_reversed_ops(&right);
    let mut left = eval_tree(&left);

    for (op, right) in ops {
        println!("{op:?}, {right}");

        left = op.operate(left, right)
    }

    left
}

fn get_reversed_ops(tree: &MonkeyTree) -> Vec<(Operation, i64)> {
    match tree {
        MonkeyTree::Node {
            left,
            right,
            operation,
        } => {
            let mut left = *left.clone();
            let mut right = *right.clone();

            // Do this so we know which one has the human: the right one always has the human
            // after this.
            if contains_human(&left) {
                swap(&mut left, &mut right);
            }

            let left = eval_tree(&left.clone());
            let mut right_ops = get_reversed_ops(&right);
            let mut ops = vec![(operation.opposite(), left)];

            ops.append(&mut right_ops);

            ops
        }
        MonkeyTree::Human => vec![],
        MonkeyTree::Leaf(..) => unreachable!("Should not be able to get to a leaf here!"),
    }
}

fn eval_tree(tree: &MonkeyTree) -> i64 {
    match tree {
        MonkeyTree::Node {
            left,
            right,
            operation,
        } => operation.operate(eval_tree(left), eval_tree(right)),
        MonkeyTree::Leaf(x) => *x,
        MonkeyTree::Human => panic!("Humans not allowed here!"),
    }
}

fn contains_human(tree: &MonkeyTree) -> bool {
    match tree {
        MonkeyTree::Node { left, right, .. } => contains_human(left) || contains_human(right),
        MonkeyTree::Leaf(..) => false,
        MonkeyTree::Human => true,
    }
}

fn collect_monkeys(monkeys: &HashMap<String, Monkey>, id: &str) -> MonkeyTree {
    let monkey = monkeys.get(id).unwrap();

    if id == "humn" {
        return MonkeyTree::Human;
    }

    match monkey {
        Monkey::Number(x) => MonkeyTree::Leaf(*x),
        Monkey::Operation {
            left,
            right,
            operation,
        } => MonkeyTree::Node {
            left: Box::new(collect_monkeys(monkeys, left)),
            right: Box::new(collect_monkeys(monkeys, right)),
            operation: *operation,
        },
    }
}

#[derive(Clone, Debug, Hash)]
enum MonkeyTree {
    Node {
        left: Box<MonkeyTree>,
        right: Box<MonkeyTree>,
        operation: Operation,
    },
    Leaf(i64),
    Human,
}

#[solution(day = "21", part = "1")]
fn part_1(input: &str) -> i64 {
    let monkeys = monkeys(input);

    eval_tree_monkey(&monkeys, "root")
}

fn eval_tree_monkey(monkeys: &HashMap<String, Monkey>, id: &str) -> i64 {
    let monkey = monkeys.get(id).unwrap();

    match monkey {
        Monkey::Number(v) => *v,
        Monkey::Operation {
            left,
            right,
            operation,
        } => operation.operate(
            eval_tree_monkey(monkeys, left),
            eval_tree_monkey(monkeys, right),
        ),
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
