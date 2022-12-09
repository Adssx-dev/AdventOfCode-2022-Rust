use std::cmp::max;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Position {
    x : i32,
    y : i32
}

impl Position {
    pub fn move_pos(&mut self, mov : &Movement) {
        self.x += mov.delta_x;
        self.y += mov.delta_y;
    }

    pub fn move_towards(&mut self, other : &Position) {
        if other.x > self.x {
            self.x += 1;
        }
        else if other.x < self.x {
            self.x -= 1;
        }
        if other.y > self.y {
            self.y += 1;
        }
        else if other.y < self.y {
            self.y -= 1;
        }
    }

    pub fn distance(&self, other : &Position) -> i32 {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

#[derive(Clone, Debug)]
struct Movement {
    delta_x : i32,
    delta_y : i32
}

impl Movement {
    pub fn new(line : &str) -> Vec<Movement> {
        let mut iter = line.chars().into_iter();
        let direction = iter.next().unwrap();
        let count = iter.skip(1).collect::<String>().parse::<usize>().unwrap();
        match direction {
            'U' => vec![Movement{delta_x : 0, delta_y : 1}; count],
            'D' => vec![Movement{delta_x : 0, delta_y : -1}; count],
            'L' => vec![Movement{delta_x : -1, delta_y : 0}; count],
            'R' => vec![Movement{delta_x : 1, delta_y : 0}; count],
            _ => panic!("Unexpected character")        
        }
    }
}

#[derive(Debug)]
struct Board {
    knots_pos : Vec<Position>,
    all_tail_positions : Vec<Position>
}

impl Board {
    pub fn new(rope_length : usize) -> Board {
        Board {
            knots_pos : vec![Position { x: 0, y: 0 }; rope_length],
            all_tail_positions : vec!()
        }
    }

    pub fn move_head (&mut self, mov : &Movement) {
        self.knots_pos[0].move_pos(mov);

        // DO not check head here !
        for k in 1..self.knots_pos.len() {
            let previous_pos = &self.knots_pos[k-1].clone();
            if self.knots_pos[k].distance(previous_pos) > 1 {
                self.knots_pos[k].move_towards(previous_pos);
                if k == self.knots_pos.len() - 1 {
                    self.record_position(&self.knots_pos[k].clone());
                }
            }
        }

    }

    pub fn unique_positions_count(&self) -> usize {
        self.all_tail_positions.len()
    }

    fn record_position(&mut self, pos : &Position) {
        let p = self.all_tail_positions.iter().find(|tail_pos| pos == *tail_pos);
        match p {
            Some(_) => {},
            None => self.all_tail_positions.push(pos.clone())
        }
    }
}

pub fn day9_pt1 () -> usize {
    let file = include_str!("../../inputs/day9.txt");

    let movements = file.lines()
        .map(|l| Movement::new(l))
        .flatten();

    let mut board = Board::new(1);

    for m in movements {
        board.move_head(&m);

        //println!("{:?}", m);
        //println!("{:?}", board);
    }
    board.unique_positions_count() + 1

}


pub fn day9_pt2 () -> usize {
    let file = include_str!("../../inputs/day9.txt");

    let movements = file.lines()
        .map(|l| Movement::new(l))
        .flatten();

    let mut board = Board::new(10);

    for m in movements {
        board.move_head(&m);
    }
    board.unique_positions_count() + 1
}



#[cfg(test)]
mod tests {
    use crate::problems::day9::*;

    #[test]
    fn day9_pt1_test() {
        let result = day9_pt1();
        assert_eq!(result, 6332);
    }

    #[test]
    fn day9_pt2_test() {
        let result = day9_pt2();
        assert_eq!(result, 2511);
    }
}