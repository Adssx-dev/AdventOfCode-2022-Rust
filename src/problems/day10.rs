enum Instruction {
    Noop,
    Addx(i32)
}

impl Instruction {
    pub fn new(line : &str) -> Vec<Instruction> {
        let first_char = line.chars().next().unwrap();
        match first_char {
            'n' => vec!(Instruction::Noop),
            'a' => vec!(Instruction::Noop, Instruction::Addx(line.split(' ').nth(1).unwrap().parse::<i32>().unwrap())),
            _ => panic!("Error : unexepcetd instruction")
        }
    }
}

struct Computer {
    instructions : Vec<Instruction>,
    current_cycle : usize,
    register_value : i32,
    signal_strengths : Vec<i32>,
    screen : Vec<bool>,
    screen_width : usize
}

impl Computer {
    pub fn new(instructions_set : &str, screen_width : usize) -> Computer {
        let mut computer = Computer {
            instructions : instructions_set.lines().map(|l| Instruction::new(l)).flatten().collect(),
            current_cycle : 1,
            register_value : 1,
            signal_strengths : vec!(),
            screen : vec!(),
            screen_width
        };
        computer.draw_next_pixel();
        computer
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
        self.draw_next_pixel();
    }
    
    pub fn is_ended (&self) -> bool{
        self.current_cycle >= self.instructions.len() 
    }
    
    pub fn total_signal_strength (&self) -> i32 {
        self.signal_strengths.iter().sum()
    }

    pub fn draw_screen(&self) {
        for row in 0..(self.screen.len() / self.screen_width) {
            for col in 0..self.screen_width  {
                print!("{}", match &self.screen[col + row * self.screen_width] {
                    true => '#',
                    false => '.'
                });
            }
            println!("");
        }
    }

    fn draw_next_pixel(&mut self) {
        self.screen.push(((self.current_cycle % self.screen_width) as i32 - self.register_value).abs() <= 1);
    }
}

pub fn day10_pt1 () -> i32 {
    let file = include_str!("../../inputs/day10.txt");

    let mut computer = Computer::new(file, 40);
    
    while !computer.is_ended() {
        computer.run_cycle();
    }
    
    computer.total_signal_strength()
}


pub fn day10_pt2 () -> usize {
    let file = include_str!("../../inputs/day10.txt");

    let mut computer = Computer::new(file, 40);
    
    while !computer.is_ended() {
        
        computer.run_cycle();
    }

    computer.draw_screen();
    0
}




#[cfg(test)]
mod tests {
    use crate::problems::day10::*;

    #[test]
    fn day10_pt1_test() {
        let result = day10_pt1();
        assert_eq!(result, 14780);
    }

    #[test]
    fn day10_pt2_test() {
        let result = day10_pt2();
        assert_eq!(result, 0); // this is a visual puzzle
    }
}