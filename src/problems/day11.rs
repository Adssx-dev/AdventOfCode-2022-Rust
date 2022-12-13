use std::cell::RefCell;

enum Operation {
    Addition(u64),
    Multiplication(u64),
    Square
}

impl Operation {
    pub fn do_operation(&self, target : u64) -> u64{
        match self {
            Operation::Addition(operand) => target + operand,
            Operation::Multiplication(operand) => target * operand,
            Operation::Square => target * target
        }
    }
}



pub struct Monkey {
    items : Vec<u64>,
    operation : Operation,
    divisible_by_condition : u64, 
    condition_true_target_monkey : usize, // monkey to which it is sent when condition is true
    condition_false_target_monkey : usize, // monkey to which it is sent when condition is false
    inspect_count : u64,
}

impl Monkey {
    pub fn new(lines : &[&str]) -> Monkey {
        Monkey {
            items : Monkey::load_items(lines[0]),
            operation : Monkey::load_operation(lines[1]),
            divisible_by_condition : Monkey::parse_last_number(lines[2]),
            condition_true_target_monkey : Monkey::parse_last_number(lines[3]) as usize,
            condition_false_target_monkey : Monkey::parse_last_number(lines[4]) as usize,
            inspect_count : 0,
        }
    }

    pub fn play_turn(&mut self, divisor : Option<u64>) -> Vec<(u64, usize)> {
        self.inspect_count += self.items.len() as u64;
        let items_copy = self.items.clone();
        self.items.clear();
        //println!("{:?}", items_copy);
        items_copy.iter().map(|item| self.inspect_item(*item, divisor))
            .map(|item| (item, self.get_target(item)))
            .collect()
    }

    pub fn give_item(&mut self, item : u64) {
        self.items.push(item);
    }

    fn get_target(&self, item : u64) -> usize {
        let division_result = item % self.divisible_by_condition;
        match division_result {
            0 => self.condition_true_target_monkey,
            _ => self.condition_false_target_monkey
        }
    }

    fn inspect_item(&self, item : u64, divisor : Option<u64>) -> u64 {
        let result = self.operation.do_operation(item);
        match divisor {
            Some(div) => result % div,
            None => result / 3
        }
    }

    fn load_items(line : &str) -> Vec<u64> {
        line.split(':')
            .last()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|item| item.trim().parse::<u64>().unwrap())
            .collect()
    }

    fn load_operation(line : &str) -> Operation {
        let mut splitted = line.split('=')
            .last()
            .unwrap()
            .split(' ')
            .skip(2);
        let op = splitted.next().unwrap();
        let value = splitted.next().unwrap().trim();
        match op {
            "+" => Operation::Addition(value.parse::<u64>().unwrap()),
            "*" => match value {
                        "old" => Operation::Square,
                        x => Operation::Multiplication(x.parse::<u64>().unwrap()),
                    }
            _ => panic!("Unexpected operation")
        }
    }

    fn parse_last_number(line : &str) -> u64 {
        line.split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    }
    
}

struct MonkeyGroup {
    monkeys : Vec<RefCell<Monkey>>,
}

impl MonkeyGroup {
    pub fn new(lines : &[&str]) -> MonkeyGroup{

        let mut monkeys = MonkeyGroup {
            monkeys : vec!()
        };
        let monkey_count = lines.len() / 7;
        for i in 0..=monkey_count {
            let line_idx = i * 7 + 1;
            monkeys.monkeys.push(RefCell::new(Monkey::new(&lines[line_idx..(line_idx+5)])));
        }
        monkeys
    }

    pub fn play_round(&mut self, divisor : Option<u64>) {
        for m in &self.monkeys {
            let items_output = m.borrow_mut().play_turn(divisor);
            for (item, target) in items_output {
                self.monkeys[target].borrow_mut().give_item(item);
            }
        }
    }
}


pub fn day11_pt1 () -> u64 {
    let file = include_str!("../../inputs/day11.txt");
    let mut monkey_group = MonkeyGroup::new(&file.split('\n').collect::<Vec<&str>>());
    for _ in 0..20 {
        monkey_group.play_round(None);
    }
    
    let mut scores  : Vec<u64> = monkey_group.monkeys.iter().map(|monkey| monkey.borrow().inspect_count).collect();
    scores.sort_unstable();
    scores.reverse();
    scores[0] * scores[1]
}


pub fn day11_pt2 () -> u64 {
    let file = include_str!("../../inputs/day11.txt");
    let mut monkey_group = MonkeyGroup::new(&file.split('\n').collect::<Vec<&str>>());

    let divisors_product : u64 = monkey_group.monkeys.iter().map(|m| m.borrow().divisible_by_condition).product();

    for _ in 0..10000 {
        monkey_group.play_round(Some(divisors_product));
    }
    
    let mut scores  : Vec<u64> = monkey_group.monkeys.iter().map(|monkey| monkey.borrow().inspect_count).collect();
    scores.sort_unstable();
    scores.reverse();
    scores[0] * scores[1]
}


#[cfg(test)]
mod tests {
    use crate::problems::day11::*;

    #[test]
    fn day11_pt1_test() {
        let result = day11_pt1();
        assert_eq!(result, 102399);
    }

    #[test]
    fn day11_pt2_test() {
        let result = day11_pt2();
        assert_eq!(result, 23641658401); 
    }
}