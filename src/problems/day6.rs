pub fn all_different(chars : &[char]) -> bool {
    let mut ok = true;
    for idx in 0..(chars.len()-1) {
        if chars[(idx+1)..].contains(&chars[idx]) {
            ok = false;
            break; // At least two are identical, we cen safely avoir remaining calculations
        }
    }
    ok
}

pub fn different_sequence(characters : Vec<char>, sequence_length : usize) -> usize {
    (0..characters.len() - (sequence_length - 1)) //Get indexes from start to end, but avoir going over the end of the indexes
        .find(|i|  all_different(&characters[(*i)..(*i+sequence_length)])) // check if all characters are different in the string
        .unwrap() + sequence_length // add the sequence_length to get the last character of the sequence
}

pub fn day6_pt1() -> usize {
    let file : Vec<char>= include_str!("../../inputs/day6.txt").chars().collect();

    different_sequence(file, 4)
}

pub fn day6_pt2() -> usize {
    let file : Vec<char>= include_str!("../../inputs/day6.txt").chars().collect();
    
    different_sequence(file, 14)
}
