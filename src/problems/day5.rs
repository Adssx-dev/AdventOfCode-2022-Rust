// Represents an instruction for the crane
struct Instruction {
    from: usize, // Stack to take from
    to: usize,   // stack to put to
    qty: usize,  // Number of items to move from A to B
}

fn load_stacks(stacks_string: &[&str]) -> Vec<Vec<char>> {
    let mut initial_stacks: Vec<Vec<char>> = stacks_string
        .iter()
        .map(|line| {
            line.chars()
                .into_iter() // Take 1 item in 4 after skipping the 1st one (remove brackets and spaces)
                .skip(1)
                .step_by(4)
                .collect::<Vec<char>>()
        })
        .collect();

    initial_stacks.reverse();

    // Transpose the vector of vectors so that each vector in the the indexes in the first vector each correspond to a stack
    (0..initial_stacks[0].len())
        .map(|i| {
            initial_stacks
                .iter()
                .map(|inner| inner[i])
                .filter(|elem| elem != &' ')
                .collect::<Vec<char>>()
        })
        .collect()
}

fn load_instructions(instructions_string: &[&str]) -> Vec<Instruction> {
    instructions_string
        .iter()
        .map(|ins| ins.split(' '))
        .map(|splitted| splitted.skip(1).step_by(2)) // Take even elements (odd elems = words, even elems = numbers)
        .map(|mut it| Instruction { // Convert to a Instruction struct
            qty: it.next().unwrap().parse::<usize>().unwrap(),
            from: it.next().unwrap().parse::<usize>().unwrap() - 1,
            to: it.next().unwrap().parse::<usize>().unwrap() - 1,
        })
        .collect()
}

pub fn day5_pt1() -> String {
    let file = include_str!("../../inputs/day5.txt");

    let empty_line = file.lines().position(|line| line.eq("")).unwrap();

    let mut stacks: Vec<Vec<char>> =
        load_stacks(&(file.lines().take(empty_line - 1).collect::<Vec<&str>>()));

    let instructions: Vec<Instruction> =
        load_instructions(&(file.lines().skip(empty_line + 1).collect::<Vec<&str>>()));

    // Pop elems from the source and push them to the destination
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

    let mut stacks: Vec<Vec<char>> =
        load_stacks(&(file.lines().take(empty_line - 1).collect::<Vec<&str>>()));

    let instructions: Vec<Instruction> =
        load_instructions(&(file.lines().skip(empty_line + 1).collect::<Vec<&str>>()));

    // Using an intermediate vector reverses two times and sets the items in the right order
    for instruction in instructions {
        let mut tmp_vec: Vec<char> = vec![];
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



#[cfg(test)]
mod tests {
    use crate::problems::day5::*;

    #[test]
    fn day5_pt1_test() {
        let result = day5_pt1();
        assert_eq!(result, "RNZLFZSJH");
    }

    #[test]
    fn day5_pt2_test() {
        let result = day5_pt2();
        assert_eq!(result, "CNSFCGJSM");
    }
}
