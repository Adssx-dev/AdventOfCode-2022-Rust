use std::cell::RefCell;

enum Operation {
    Addition(u32),
    Multiplication(u32),
    Square
}

impl Operation {
    pub fn do_operation(&self, target : u32) -> u32{
        match self {
            Operation::Addition(operand) => target + operand,
            Operation::Multiplication(operand) => target * operand,
            Operation::Square => target * target
        }
    }
}

#[derive(Clone)]
struct Item {
    item_modulos : Vec<u32>
}

impl Item {
    pub fn new(initial_value : u32, modulo_count : usize) -> Item {
        Item {
            item_modulos : vec!(initial_value; modulo_count),
        }
    }
}

pub struct Monkey {
    items : Vec<Item>,
    operation : Operation,
    divisible_by_condition : u32, 
    condition_true_target_monkey : usize, // monkey to which it is sent when condition is true
    condition_false_target_monkey : usize, // monkey to which it is sent when condition is false
    inspect_count : u32,
    all_divisors : Vec<u32>,
    index : usize
}

impl Monkey {
    pub fn new(lines : &[&str], monkey_count : usize, monkey_index : usize) -> Monkey {
        Monkey {
            items : Monkey::load_items(lines[0], monkey_count),
            operation : Monkey::load_operation(lines[1]),
            divisible_by_condition : Monkey::parse_last_number(lines[2]),
            condition_true_target_monkey : Monkey::parse_last_number(lines[3]) as usize,
            condition_false_target_monkey : Monkey::parse_last_number(lines[4]) as usize,
            inspect_count : 0,
            all_divisors : vec!(),
            index : monkey_index
        }
    }

    pub fn play_turn(&mut self) -> Vec<(Item, usize)> {
        self.inspect_count += self.items.len() as u32;
        let items_copy = self.items.clone();
        self.items.clear();
        //println!("{:?}", items_copy);
        items_copy.iter().map(|item| self.inspect_item(item.clone()))
            .map(|item| (item.clone(), self.get_target(item)))
            .collect()
    }

    pub fn give_item(&mut self, item : Item) {
        self.items.push(item);
    }

    fn get_target(&self, item : Item) -> usize {
        let division_result = item.item_modulos[self.index];
        match division_result {
            0 => self.condition_true_target_monkey,
            _ => self.condition_false_target_monkey
        }
    }

    fn inspect_item(&self, mut item : Item) -> Item{

        for idx in 0..item.item_modulos.len() {
            item.item_modulos[idx] = (self.operation.do_operation(item.item_modulos[idx]) / 3) % self.all_divisors[idx];
        }
        item
    }

    fn load_items(line : &str, monkey_count : usize) -> Vec<Item> {
        line.split(':')
            .last()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|item| Item::new(item.trim().parse::<u32>().unwrap(), monkey_count))
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
            "+" => Operation::Addition(value.parse::<u32>().unwrap()),
            "*" => match value {
                        "old" => Operation::Square,
                        x => Operation::Multiplication(x.parse::<u32>().unwrap()),
                    }
            x => panic!("Unexpected operation".to_owned() + x)
        }
    }

    fn parse_last_number(line : &str) -> u32 {
        line.split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<u32>()
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
            monkeys.monkeys.push(RefCell::new(Monkey::new(&lines[line_idx..(line_idx+5)], monkey_count+1, i)));
        }
        let all_modulos = monkeys.monkeys.iter().map(|m| m.borrow().divisible_by_condition).collect::<Vec<u32>>();
        for m in &monkeys.monkeys {
            m.borrow_mut().all_divisors = all_modulos.clone();
        }
        monkeys
    }

    pub fn play_round(&mut self) {
        for m in &self.monkeys {
            let items_output = m.borrow_mut().play_turn();
            for (item, target) in items_output {
                self.monkeys[target].borrow_mut().give_item(item);
            }
        }
    }
}


pub fn day11_pt1 () -> u32 {
    let file = include_str!("../../inputs/day11.txt");
    let mut monkey_group = MonkeyGroup::new(&file.split('\n').collect::<Vec<&str>>());
    for i in 0..20 {
        monkey_group.play_round();
    }
    
    let mut scores  : Vec<u32> = monkey_group.monkeys.iter().map(|monkey| monkey.borrow().inspect_count).collect();
    scores.sort();
    scores.reverse();
    scores[0] * scores[1]
}


pub fn day11_pt2 () -> usize {
    let file = include_str!("../../inputs/day11.txt");

    0
}




#[cfg(test)]
mod tests {
    use crate::problems::day11::*;

    #[test]
    fn day11_pt1_test() {
        let result = day11_pt1();
        assert_eq!(result, 0);
    }

    #[test]
    fn day11_pt2_test() {
        let result = day11_pt2();
        assert_eq!(result, 0); // this is a visual puzzle
    }
}