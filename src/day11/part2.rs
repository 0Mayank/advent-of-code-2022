use crate::utils::read_lines;
use std::cell::RefCell;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "*" => Ok(Self::Mul),
            "+" => Ok(Self::Add),
            other => Err(format!("invalid operator: {other}")),
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    receiver_if_true: usize,
    reciever_if_false: usize,
}

impl Test {
    fn test(&self, other: u64) -> usize {
        if other % self.divisor == 0 {
            return self.receiver_if_true;
        }
        return self.reciever_if_false;
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspected_items: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: vec![],
            operation: Operation {
                operator: Operator::Mul,
                operand: None,
            },
            test: Test {
                divisor: 1,
                receiver_if_true: 0,
                reciever_if_false: 0,
            },
            inspected_items: 0,
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operand: Option<u64>,
}

impl Operation {
    fn perform(&self, other: u64) -> u64 {
        if let Some(operand) = self.operand {
            match self.operator {
                Operator::Mul => operand * other,
                Operator::Add => operand + other,
            }
        } else {
            match self.operator {
                Operator::Mul => other * other,
                Operator::Add => other + other,
            }
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<RefCell<Monkey>>,
    round: u64,
}

impl Monkeys {
    fn new() -> Self {
        Self {
            monkeys: vec![],
            round: 0,
        }
    }

    fn next_round(&mut self, divisor_product: u64) {
        self.round += 1;
        for monkey in 0..self.monkeys.len() {
            let mut monkey = self.monkeys[monkey].borrow_mut();
            while let Some(mut item) = monkey.items.pop() {
                monkey.inspected_items += 1;
                item = monkey.operation.perform(item) % divisor_product;
                let monkey_no = monkey.test.test(item);
                self.monkeys[monkey_no].borrow_mut().items.push(item);
            }
        }
    }

    fn monkey_business(&mut self, no_of_rounds: u64) -> u64 {
        let divisor_product = self
            .monkeys
            .iter()
            .map(|x| x.borrow_mut().test.divisor)
            .product();

        while self.round < no_of_rounds {
            self.next_round(divisor_product);
        }

        let mut inspected_items = vec![];
        for monkey in &self.monkeys {
            inspected_items.push(monkey.borrow().inspected_items);
        }

        inspected_items.sort_by(|a, b| b.cmp(a));

        inspected_items[0] * inspected_items[1]
    }

    fn parse<I>(&mut self, lines: &mut I) -> std::io::Result<()>
    where
        I: Iterator<Item = std::io::Result<String>>,
    {
        let mut current_monkey = 0;

        for line in lines {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            let mut line = line.trim().split(':');

            match line.next() {
                Some("Starting items") => {
                    if let Some(items) = line.next() {
                        for item in items.split(&[' ', ',']) {
                            if !item.is_empty() {
                                self.monkeys[current_monkey].borrow_mut().items.push(
                                    item.parse().expect("Item worry level should be an integer"),
                                );
                            }
                        }
                    }
                }
                Some("Operation") => {
                    let mut op = line
                        .next()
                        .expect("No operation provided")
                        .split("old")
                        .nth(1)
                        .expect("No operation provided")
                        .split_whitespace();

                    self.monkeys[current_monkey].borrow_mut().operation.operator =
                        op.next().unwrap().try_into().unwrap();
                    let operand = op.next();

                    if let Some(operand) = operand {
                        self.monkeys[current_monkey].borrow_mut().operation.operand =
                            Some(operand.parse().expect("Provided operand is not an integer"));
                    } else {
                        self.monkeys[current_monkey].borrow_mut().operation.operand = None;
                    }
                }
                Some("Test") => {
                    self.monkeys[current_monkey].borrow_mut().test.divisor = line
                        .next()
                        .expect("Test condition not provided")
                        .split_whitespace()
                        .next_back()
                        .expect("Test number not provided")
                        .parse()
                        .expect("Provided test not a number");
                }
                Some("If true") => {
                    self.monkeys[current_monkey]
                        .borrow_mut()
                        .test
                        .receiver_if_true = line
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .next_back()
                        .expect("Case true monkey number not provided")
                        .parse::<usize>()
                        .expect("Case true monkey no. should be an integer");
                }
                Some("If false") => {
                    self.monkeys[current_monkey]
                        .borrow_mut()
                        .test
                        .reciever_if_false = line
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .next_back()
                        .expect("Case true monkey number not provided")
                        .parse::<usize>()
                        .expect("Case true monkey no. should be an integer");
                }

                Some(other) => {
                    let mut other = other.split_whitespace();
                    match other.next() {
                        Some("Monkey") => {
                            self.monkeys.push(RefCell::new(Monkey::new()));
                            current_monkey = self.monkeys.len() - 1;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}

pub fn run() -> String {
    let mut monkeys = Monkeys::new();
    let mut lines = read_lines("11").unwrap();
    monkeys.parse(&mut lines).unwrap();

    monkeys.monkey_business(10000).to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(16792940265u64.to_string(), run());
    }
}
