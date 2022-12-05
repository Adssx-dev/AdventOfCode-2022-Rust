


struct Instruction {
    from : usize,
    to : usize,
    qty : usize
}

fn load_stacks(stacks_string : &[&str]) -> Vec<Vec<char>> {
    let mut initial_stacks : Vec<Vec<char>> = stacks_string.iter()
        .map(|line| line.chars().into_iter()
            .skip(1)
            .step_by(4)
            .collect::<Vec<char>>()
            )
        .collect();

    initial_stacks.reverse();

     (0..initial_stacks[0].len())
        .map(|i| initial_stacks.iter().map(|inner| inner[i])
            .filter(|elem| elem != &' ')
            .collect::<Vec<char>>())
        .collect()
}

fn load_instructions(instructions_string : &[&str]) -> Vec<Instruction> {
    instructions_string.iter()
        .map(|ins| ins.split(' '))
        .map(|splitted| splitted.skip(1)
            .step_by(2))
        .map(|mut splitted| (splitted.next().unwrap(), splitted.next().unwrap(), splitted.next().unwrap()))
        .map(|(s1, s2, s3)| Instruction {
                qty : s1.parse::<usize>().unwrap(),
                from: s2.parse::<usize>().unwrap() - 1,
                to: s3.parse::<usize>().unwrap() - 1
            }
        )
        .collect()
}

pub fn day5_pt1() -> String {
    
    let file = include_str!("../../inputs/day5.txt");

    let empty_line = file.lines().position(|line| line.eq("")).unwrap();

    let mut stacks : Vec<Vec<char>> = load_stacks(&(file.lines().take(empty_line - 1).collect::<Vec<&str>>()));

    let instructions : Vec<Instruction> = load_instructions(&(file.lines().skip(empty_line + 1).collect::<Vec<&str>>()));


    for instruction in instructions {
        for _ in 0..instruction.qty {
            let elem = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(elem);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()

}


pub fn day5_pt2() -> String {
    
    let file = include_str!("../../inputs/day5.txt");

    let empty_line = file.lines().position(|line| line.eq("")).unwrap();

    let mut stacks : Vec<Vec<char>> = load_stacks(&(file.lines().take(empty_line - 1).collect::<Vec<&str>>()));

    let instructions : Vec<Instruction> = load_instructions(&(file.lines().skip(empty_line + 1).collect::<Vec<&str>>()));


    for instruction in instructions {
        let mut tmp_vec : Vec<char> = vec!();
        for _ in 0..instruction.qty {
            let elem = stacks[instruction.from].pop().unwrap();
            tmp_vec.push(elem);
        }

        for _ in 0..instruction.qty {
            let elem = tmp_vec.pop().unwrap();
            stacks[instruction.to].push(elem);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()

}


