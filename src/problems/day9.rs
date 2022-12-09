use std::cmp::max;

// Represents a position on the grid.
// x and Y are i32 as it can go negative. This is not an issue as it is never used to index anything
#[derive(PartialEq, Eq, Clone, Debug)]
struct Position {
    x : i32,
    y : i32
}

impl Position {
    // Move the current position according to a Movement
    pub fn move_pos(&mut self, mov : &Movement) {
        self.x += mov.delta_x;
        self.y += mov.delta_y;
    }

    // Move the current position towards another one
    pub fn move_towards(&mut self, other : &Position) {
        self.x += match other.x - self.x {
            v if v < 0 => -1,
            v if v > 0 => 1,
            _  => 0
        };

        self.y += match other.y - self.y {
            v if v < 0 => -1,
            v if v > 0 => 1,
            _  => 0
        };
    }

    // Calculate the distance between two points.
    // In this problem the distance not the euclidian distance but rather the greatest between the x distance and the y distance
    pub fn distance(&self, other : &Position) -> i32 {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

// Describes a movement (x distance + y distance)
#[derive(Clone, Debug)]
struct Movement {
    delta_x : i32,
    delta_y : i32
}

impl Movement {
    pub fn new(line : &str) -> Vec<Movement> {
        let mut iter = line.chars();
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

// COntains nodes to move + record the positions of the tail
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

    // Move the head, then check if other nodes need to move (everything is cascading)
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

    // Count the number of unique positions the tail has been on
    pub fn unique_positions_count(&self) -> usize {
        self.all_tail_positions.len()
    }

    // Saves the position of the tail to the list if it does not already exist
    // This si definitely not optimised, as is is stored in a vec without ordering
    // Using a set would be A LOT better
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
        .map(|l| Movement::new(l)) // generate the list of N movements for each line
        .flatten(); // Convert all to a 1D list

    let mut board = Board::new(2);

    for m in movements {
        board.move_head(&m); 
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