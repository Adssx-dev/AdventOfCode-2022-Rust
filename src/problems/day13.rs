use std::{str::Chars, cmp::{Ordering, max}};

#[derive(Debug, Clone)]
enum Element {
    Scalar(u32),
    Vector(List),
}

impl Element {
    pub fn ordered(&self, other : &Element) -> Ordering {
        match (self, other) {
            (Element::Scalar(s1), Element::Scalar(s2)) => s1.cmp(s2),
            (Element::Scalar(_), Element::Vector(_)) => self.to_vector().ordered(other),
            (Element::Vector(_), Element::Scalar(_)) => self.ordered(&other.to_vector()),
            (Element::Vector(v1), Element::Vector(v2)) => {
                let mut ord : Ordering = Ordering::Equal;
                for i in 0..max(v1.elements.len(), v2.elements.len()) {
                    if i >= v1.elements.len() {
                        ord = Ordering::Less;
                        break;
                    }
                    else if i >= v2.elements.len() {
                        ord = Ordering::Greater;
                        break;
                    }
                    let ord_tmp = v1.elements[i].ordered(&v2.elements[i]);
                    if ord_tmp != Ordering::Equal {
                        ord = ord_tmp;
                        break;
                    }
                }
                ord
            }

        }
    }

    fn to_vector(&self) -> Element {
        match self {
            Element::Scalar(s) => Element::Vector(List{elements: vec!(Element::Scalar(*s))}),
            Element::Vector(v) => self.clone()
        }
    }
}


#[derive(Debug, Clone)]
struct List {
    elements: Vec<Element>,
}

impl List {
    pub fn new(line: &[char]) -> List {
        let mut list = List { elements: vec![] };
        //println!("{:?}", line);
        
        if line.len() > 0 {
            let mut idx = 0;
            let mut buffer = Vec::<char>::new();
        
            loop {
                if idx == line.len() {
                    if buffer.len() > 0 {
                        list.elements.push(Element::Scalar(
                            buffer.iter().collect::<String>().parse::<u32>().unwrap(),
                        ));
                    }
                    break;
                }
                //if idx >= str_length { break; }
                let current_char = line[idx];
                match current_char {
                    '[' => {
                        let matching_bracket = List::find_matching_bracket(line, idx);
                        list.elements.push(Element::Vector(List::new(
                            &line[(idx + 1)..matching_bracket],
                        )));
                        idx = matching_bracket + 1;
                    }
                    ',' => {
                        if buffer.len() > 0 {
                            list.elements.push(Element::Scalar(
                                buffer.iter().collect::<String>().parse::<u32>().unwrap(),
                            ));
                            buffer.clear();
                        }
                        idx += 1;
                    }
                    c if c >= '0' && c <= '9' => {
                        buffer.push(c);
                        idx += 1;
                    }
                    '\r' => {idx += 1}
                    a => {
                        print!("{}", a);
                        panic!("Unexpected character");
                    }
                }
            }
        }
        list
    }

    fn find_matching_bracket(line: &[char], start_index: usize) -> usize {
        let mut deepness = 0;
        let mut idx = start_index;
        loop {
            match line[idx] {
                '[' => deepness += 1,
                ']' => deepness -= 1,
                _ => {}
            }
            if deepness == 0 {
                break;
            }
            idx += 1;
        }
        idx
    }
}

pub fn day13_pt1() -> usize {
    let file = include_str!("../../inputs/day13.txt");
    let test_line: Vec<char> = "[[[2]],[2,[3],[9,3],10],8],[[9,[5,7,5,5],6,8],[[],7,7,2]],[[]]"
        .chars()
        .collect();
    let test_line2: Vec<char> = "[[[0,[5,6,5],[0,4,1]],[],[3],[]],[5,0,1],[1,3,6,[1,[7,4]],10],[]]"
        .chars()
        .collect();
    let test_line: Vec<char> = "[[[]]]"
        .chars()
        .collect();
    let test_line2: Vec<char> = "[[]]"
        .chars()
        .collect();
    let test = Element::Vector(List::new(&test_line));
    let test2 = Element::Vector(List::new(&test_line2));
    let splitted = file.split('\n').collect::<Vec<&str>>();
    (0..(splitted.len() / 3)) 
        .map(|i| (i, splitted[i * 3].chars().collect::<Vec<char>>(), splitted[i * 3+1].chars().collect::<Vec<char>>()))
        .map(|(i, s1, s2)| (i, Element::Vector(List::new(&s1)), Element::Vector(List::new(&s2))))
        .map(|(i, e1, e2)| (i, e1.ordered(&e2)))
        .filter(|(i, order)| *order == Ordering::Less)
        .map(|(i, order)| i+1)
        .sum()
    //println!("{:?}", test.ordered(&test2));

    //print!("{:?}", test.elements);
}

pub fn day13_pt2() -> u32 {
    let file = include_str!("../../inputs/day13.txt");

    0
}

#[cfg(test)]
mod tests {
    use crate::problems::day13::*;

    #[test]
    fn day13_pt1_test() {
        let result = day13_pt1();
        assert_eq!(result, 0);
    }

    #[test]
    fn day13_pt2_test() {
        let result = day13_pt2();
        assert_eq!(result, 0);
    }
}
