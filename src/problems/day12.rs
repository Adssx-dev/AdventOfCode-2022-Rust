use std::collections::{VecDeque, vec_deque};

struct Coordinates {
    x : usize,
    y : usize
}

struct Grid {
    distance_map : Vec<u16>,
    height_map : Vec<char>,
    height : usize,
    width : usize,
}

impl Grid {
    pub fn new(text : &str) -> Grid {
        
        let height_map : Vec<char> = text.chars().filter(|c| *c != '\r' && *c != '\n').collect();
        let elem_count = height_map.len();
        let width = text.chars().filter(|c| *c != '\r').position(|c| c == '\n').unwrap();
        let height = elem_count / width;
        Grid {
            distance_map : vec!(u16::MAX; elem_count),
            height_map,
            height,
            width
        }
    }

    fn get_start_point(&self) -> Coordinates {
        let idx = self.height_map.iter().position(|c| *c == 'S').unwrap();
        Coordinates {
            x : idx % self.width,
            y : idx / self.width
        }
    }

    pub fn set_distance_map(&mut self, pos : &Coordinates, distance : u16) {
        self.distance_map[pos.y * self.width + pos.x] = distance;
    }
    

    fn get_end_point(&self) -> Coordinates {
        let idx = self.height_map.iter().position(|c| *c == 'E').unwrap();
        Coordinates {
            x : idx % self.width,
            y : idx / self.width
        }
    }

    fn get_index_in_arrays(&self, pos : &Coordinates) -> usize {
        pos.y * self.width + pos.x
    }

    fn get_height(&self, pos : &Coordinates) -> i8 {
        Grid::char_to_height(self.height_map[self.get_index_in_arrays(pos)])
    }

    fn get_distance(&self, pos : &Coordinates) -> u16 {
        self.distance_map[self.get_index_in_arrays(pos)]
    }

    fn set_distance(&mut self, pos : &Coordinates, value : u16)  {
        let idx = self.get_index_in_arrays(pos);
        self.distance_map[idx] = value;
    }

    fn char_to_height (ch : char) -> i8 {
        match ch {
            'S' => 0,
            'E' => 25,
            other => other as i8 - 'a' as i8
        }
    }

    fn get_neighbors(&self, pos : &Coordinates) -> Vec<Coordinates> {
        let mut result : Vec<Coordinates> = vec!();
        if pos.x > 0 {
            result.push(Coordinates { x : pos.x - 1, y : pos.y});
        }
        if pos.x < self.width - 1 {
            result.push(Coordinates { x : pos.x + 1, y : pos.y});
        }
        if pos.y > 0 {
            result.push(Coordinates { x : pos.x, y : pos.y - 1});
        }
        if pos.y < self.height - 1 {
            result.push(Coordinates { x : pos.x, y : pos.y + 1});
        }
        result
    }

    pub fn generate_distance_map(&mut self) {
        let start = self.get_start_point();
        self.set_distance(&start, 0);
        let mut queue : VecDeque<Coordinates> = VecDeque::new();
        queue.push_back(start);

        while !queue.is_empty() {
            let current_position = queue.pop_front().unwrap();
            let current_height = self.get_height(&current_position);
            let current_distance = self.get_distance(&current_position);
            let new_distance = current_distance + 1;

            let neighbors = self.get_neighbors(&current_position);
            for neighbor in neighbors {
                let neighbor_height = self.get_height(&neighbor);
                let neighbor_distance = self.get_distance(&neighbor);

                if neighbor_height - current_height <= 1 &&
                        neighbor_distance > new_distance {
                    self.set_distance(&neighbor, new_distance);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    pub fn get_end_distance(&self) -> u16 {
        let end_position = self.get_end_point();
        self.get_distance(&end_position)
    }

    pub fn print_distances(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{} ", self.distance_map[y * self.width + x]);
            }
            println!("");
        }
    }


}


pub fn day12_pt1 () -> u16 {
    let file = include_str!("../../inputs/day12.txt");
//     "Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi";
    
    let mut grid = Grid::new(file);
    grid.generate_distance_map();
    grid.print_distances();
    grid.get_end_distance()
}


pub fn day12_pt2 () -> u64 {
    let file = include_str!("../../inputs/day12.txt");
    
    0
}


#[cfg(test)]
mod tests {
    use crate::problems::day12::*;

    #[test]
    fn day12_pt1_test() {
        let result = day12_pt1();
        assert_eq!(result, 0);
    }

    #[test]
    fn day12_pt2_test() {
        let result = day12_pt2();
        assert_eq!(result, 23641658401); 
    }
}