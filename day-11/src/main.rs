use advent_utils::files::read;
use anyhow::Result;

fn main() -> Result<()> {
    let input = read("day-11/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let mut monkeys = monkeys(input);

    let modulo = lcm(&monkeys
        .iter()
        .map(|m| m.test.divisible_by)
        .collect::<Vec<_>>());

    // Run the simulation for 20 rounds
    for _ in 0..10000 {
        run_worried_round(&mut monkeys, modulo);
    }

    let mut inspected_times = monkeys
        .iter()
        .map(|m| m.inspected_times)
        .collect::<Vec<_>>();

    // Reverse sort the inspected times so we can take the two highest
    inspected_times.sort_by(|a, b| b.cmp(a));

    inspected_times.iter().take(2).product()
}

fn lcm(numbers: &[usize]) -> usize {
    let mut lcm = 1;
    for &number in numbers {
        lcm = lcm * number / gcd(lcm, number);
    }
    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Like `run_round` but no dividing item by 3 after inspection
fn run_worried_round(monkeys: &mut [Monkey], modulo: usize) {
    // Clone the monkeys so we can iterate over them while mutating the actual
    // monkeys.
    #[allow(clippy::unnecessary_to_owned)]
    for index in 0..monkeys.len() {
        if monkeys[index].items.is_empty() {
            continue;
        }

        for _ in 0..monkeys[index].items.len() {
            let mut item = monkeys[index].items.remove(0);

            // Calculate the stress level on the item
            item = monkeys[index].operation.run(item);

            // Stress relief?
            item %= modulo;

            // Update the inspected times counter
            monkeys[index].inspected_times += 1;

            // Throw the item to the next monkey
            let next_monkey = monkeys[index].test.test(item);
            monkeys[next_monkey].items.push(item);
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut monkeys = monkeys(input);

    // Run the simulation for 20 rounds
    for _ in 0..20 {
        run_round(&mut monkeys);
    }

    let mut inspected_times = monkeys
        .iter()
        .map(|m| m.inspected_times)
        .collect::<Vec<_>>();

    // Reverse sort the inspected times so we can take the two highest
    inspected_times.sort_by(|a, b| b.cmp(a));

    inspected_times.iter().take(2).product()
}

fn run_round(monkeys: &mut [Monkey]) {
    // Clone the monkeys so we can iterate over them while mutating the actual
    // monkeys.
    #[allow(clippy::unnecessary_to_owned)]
    for index in 0..monkeys.len() {
        if monkeys[index].items.is_empty() {
            continue;
        }

        for _ in 0..monkeys[index].items.len() {
            let mut item = monkeys[index].items.remove(0);

            // Calculate the stress level on the item
            item = monkeys[index].operation.run(item);

            // Update the inspected times counter
            monkeys[index].inspected_times += 1;

            // Relieve (some) of the stress
            item /= 3;

            // Throw the item to the next monkey
            let next_monkey = monkeys[index].test.test(item);
            monkeys[next_monkey].items.push(item);
        }
    }
}

fn monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_monkey).collect()
}

fn parse_monkey(input: &str) -> Monkey {
    let lines = input
        .lines()
        // Skip the first b/c it's the monkey number
        .skip(1)
        .collect::<Vec<_>>();

    let items = lines[0]
        .split_whitespace()
        .skip(2)
        .collect::<String>()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let operation = {
        let expr = lines[1].split_whitespace().skip(3).collect::<Vec<_>>();
        let operator = match expr[1] {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            op => unreachable!("Unknown operator '{op}'"),
        };
        let right = match expr[2] {
            "old" => Value::Item,
            number => Value::Number(number.parse().unwrap()),
        };

        Operation { operator, right }
    };

    let test = {
        let test = &lines[2..];
        let divisible_by = test[0][21..].parse().unwrap();
        let if_true = test[1][29..].parse().unwrap();
        let if_false = test[2][30..].parse().unwrap();

        Test {
            divisible_by,
            if_true,
            if_false,
        }
    };

    Monkey {
        items,
        operation,
        test,
        inspected_times: 0,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspected_times: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Test {
    divisible_by: usize,

    // The monkey to throw to if the test passes
    if_true: usize,

    // The monkey to throw to if the test fails
    if_false: usize,
}

impl Test {
    fn test(&self, item: usize) -> usize {
        if item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Operation {
    operator: Operator,
    right: Value,
}

impl Operation {
    fn run(&self, item: usize) -> usize {
        let left = item;
        let right = match self.right {
            Value::Number(number) => number,
            Value::Item => item,
        };
        match self.operator {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Value {
    Number(usize),
    Item,
}
