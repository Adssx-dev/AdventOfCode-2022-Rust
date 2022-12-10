enum Instruction {
    Noop,
    Addx(i32)
}

impl Instruction {
    pub fn new(line : &str) -> Vec<Instruction> {
        let first_char = line.chars().next().unwrap();
        match first_char {
            'n' => vec!(Instruction::Noop),
            'a' => vec!(Instruction::Noop, Instruction::Addx(line.split(' ').skip(1).next().unwrap().parse::<i32>().unwrap())),
            _ => panic!("Error : unexepcetd instruction")
        }
    }

    pub fn time_cost(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2
        }
    }
}

struct Computer {
    instructions : Vec<Instruction>,
    current_cycle : usize,
    register_value : i32,
    signal_strengths : Vec<i32>
}

impl Computer {
    pub fn new(instructions_set : &str) -> Computer {
        Computer {
            instructions : instructions_set.lines().map(|l| Instruction::new(l)).flatten().collect(),
            current_cycle : 1,
            register_value : 1,
            signal_strengths : vec!()
        }
    }

    pub fn run_cycle(&mut self) {
        self.current_cycle += 1;
        if self.current_cycle % 40 == 20 {
            self.signal_strengths.push(self.register_value * self.current_cycle as i32)
        }
        match self.instructions[self.current_cycle - 1] {
            Instruction::Noop => {},
            Instruction::Addx(add) => self.register_value += add
        }
    }

    pub fn is_ended (&self) -> bool{
        self.current_cycle >= self.instructions.len() - 1
    }

    pub fn total_signal_strength (&self) -> i32 {
        self.signal_strengths.iter().sum()
    }
}

pub fn day10_pt1 () -> i32 {
    let file = include_str!("../../inputs/day10.txt");

    let mut computer = Computer::new(file);
    
    while !computer.is_ended() {
        computer.run_cycle();
    }
    
    computer.total_signal_strength()
}


pub fn day10_pt2 () -> usize {
    let file = include_str!("../../inputs/day10.txt");

    0
}
