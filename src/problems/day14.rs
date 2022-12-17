#[derive(PartialEq, Eq, Clone, Copy)]
struct Coordinates {
    x : usize,
    y : usize
}

impl Coordinates {
    pub fn get_direction (&self, other : &Coordinates) -> (i32, i32) {
        let x = match other.x as i32 - self.x as i32 {
            val if val < 0 => -1,
            val if val == 0 => 0,
            val if val > 0 => 1,
            _ => panic!("Unereachable")
        };

        let y = match other.y as i32 - self.y as i32 {
            val if val < 0 => -1,
            val if val == 0 => 0,
            val if val > 0 => 1,
            _ => panic!("Unereachable")
        };

        (x, y)
    }
}


struct Line {
    points : Vec<Coordinates>,
}

impl Line {
    pub fn new(line_str : &str) -> Line {
        Line {
            points : line_str
                .split(" -> ")
                .map(|section| {
                    let mut split = section.split(',');
                    (split.next().unwrap(), split.next().unwrap())
                })
                .map(|(sx, sy)| Coordinates {x : sx.parse::<usize>().unwrap(), y : sy.parse::<usize>().unwrap()})
                .collect()
        }
    }

    pub fn get_all_points(&self) -> Vec<Coordinates>{
        let mut coords : Vec<Coordinates> = vec!();
        let mut points_iter = self.points.iter();

        let mut current_pos = points_iter.next().unwrap().clone();
        let mut next_pos = points_iter.next().unwrap();

        
        coords.push(current_pos.clone());
        
        let mut direction = current_pos.get_direction(&next_pos);

        loop {
            current_pos.x = (current_pos.x as i32 + direction.0) as usize;
            current_pos.y = (current_pos.y as i32 + direction.1) as usize;

            coords.push(current_pos);

            if current_pos == *next_pos {
                let next_pos_opt = points_iter.next();
                match next_pos_opt {
                    None => break,
                    Some(pos) => {
                        next_pos = pos;
                        direction = current_pos.get_direction(&next_pos);
                    }
                }
            }
        }

        coords
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellType {
    Empty,
    Wall,
    Sand
}

impl CellType {
    pub fn to_char(&self) -> char {
        match self {
            CellType::Empty => '.',
            CellType::Wall => '#',
            CellType::Sand => 'o'
        }
    }
}

// (0, 0) is the top-left corner
struct Grid {
    height : usize,
    width : usize,
    cells : Vec<CellType>
}

impl Grid {

    pub fn new(width : usize, height : usize) -> Grid  {
        Grid {
            height, 
            width, 
            cells : vec![CellType::Empty; height * width]
        }
    }

    pub fn set_walls(&mut self, walls : &[Coordinates]) {
        for wall in walls {
            let index = self.get_index(wall);
            self.cells[index] = CellType::Wall;
        }
    }

    pub fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(&Coordinates{x, y});
                print!("{}", self.cells[index].to_char());
            }
            println!("");
        }
    }

    fn get_index(&self, point :&Coordinates) -> usize {
        point.y * self.width + point.x
    }

    fn is_empty(&self, position : Coordinates) -> bool {
        let cell = self.cells[self.get_index(&position)];
        cell == CellType::Empty
    }

    pub fn drop_sand(&mut self, initial_position : Coordinates) -> Option<Coordinates> {
        let mut position = initial_position;

        loop {
            if position.y > 190 {
                return None;
            }
            let candidate_1 = Coordinates {x : position.x, y : position.y + 1};
            let candidate_2 = Coordinates {x : position.x - 1, y : position.y + 1};
            let candidate_3 = Coordinates {x : position.x + 1, y : position.y + 1};
            if self.is_empty(candidate_1) {
                position = candidate_1;
            }
            else if self.is_empty(candidate_2) {
                position = candidate_2;
            }
            else if self.is_empty(candidate_3) {
                position = candidate_3;
            }
            else {
                let index = self.get_index(&position);
                self.cells[index] = CellType::Sand;
                return Some(position);
            }
        }
    }

    pub fn drop_all_sand(&mut self, initial_position : Coordinates) -> u32 {
        let mut count : u32 = 0;
        while let Some(pos) = self.drop_sand(initial_position) {
            count += 1;
            if pos == initial_position {
                break;
            }
        }
        count
    }

    pub fn set_floor(&mut self, y_position : usize) {
        for x in 0..self.width {
            let idx = self.get_index(&Coordinates{x, y : y_position });
            self.cells[idx] = CellType::Wall;
        }
    }

}

pub fn day14_pt1() -> u32{
    let file = include_str!("../../inputs/day14.txt");

    let mut grid = Grid::new(700, 200);

    grid.set_walls(&file.split('\n')
        .map(|line_str| Line::new(line_str))
        .map(|line| line.get_all_points())
        .flatten()
        .collect::<Vec<Coordinates>>());
    let display = false ;
    if display {
        grid.display();
    }
    grid.drop_all_sand(Coordinates {x : 500, y : 0})
}

pub fn day14_pt2() -> u32 {
    let file = include_str!("../../inputs/day14.txt");

    let mut grid = Grid::new(1000, 200);

    let walls_coordinates = file.split('\n')
        .map(|line_str| Line::new(line_str))
        .map(|line| line.get_all_points())
        .flatten()
        .collect::<Vec<Coordinates>>(); 
    
    let max_y = walls_coordinates.iter()
        .map(|c| c.y)
        .max()
        .unwrap();

    grid.set_floor(max_y + 2);

    grid.set_walls(&walls_coordinates);

    let display = false ;
    if display {
        grid.display();
    }
    
    grid.drop_all_sand(Coordinates {x : 500, y : 0})
}

#[cfg(test)]
mod tests {
    use crate::problems::day14::*;

    #[test]
    fn day14_pt1_test() {
        let result = day14_pt1();
        assert_eq!(result, 755);
    }

    #[test]
    fn day14_pt2_test() {
        let result = day14_pt2();
        assert_eq!(result, 29805);
    }
}
