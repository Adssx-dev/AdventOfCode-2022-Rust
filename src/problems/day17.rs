use std::cmp::min;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Coordinates {
    x : usize,
    y : usize
}

#[derive(Clone, Copy, Default)]
struct Piece {
    position : Coordinates,
    parts_positions : [Option<Coordinates>; 5]
}

impl Piece {
    pub fn get_max_x(&self) -> usize {
        self.parts_positions.iter()
            .map(|part| match part {
                None => 0,
                Some(coord) => coord.x
            })
            .max()
            .unwrap()
    }
    
    pub fn apply_wind(&self, direction : Direction) -> Piece {
        let mut new_piece = *self;
        match direction {
            Direction::Left => {
                if new_piece.position.x > 0 {
                    new_piece.position.x -= 1;
                }
            }
            Direction::Right => {
                if new_piece.position.x + new_piece.get_max_x() < 6 {
                    new_piece.position.x += 1;
                }
            }
        }
        new_piece
    }


    pub fn fall(&self) -> Piece {
        let mut new_piece = *self;
        if new_piece.position.y > 0 {
            new_piece.position.y -= 1;
        }

        new_piece
    }
}

struct Grid {
    lines : Vec<[bool; 7]>,
    winds : Vec<Direction>,
    wind_index : usize
}

impl Grid {
    pub fn fall_piece(&mut self, initial_piece : &Piece) {
        let mut piece = *initial_piece;
        piece.position.x = 2;
        let max_y = self.top_line_index();
        piece.position.y = match max_y {
            None => 3,
            Some(y) => y + 4
        };
        let start_wind_index = self.wind_index;
        loop {
            let wind_index = self.wind_index % self.winds.len();
            let wind = self.winds[wind_index];
            //println!("Wind idx : {}, wind : {:?}", wind_index, wind);
            let winded_piece = piece.apply_wind(self.winds[self.wind_index % self.winds.len()]);
            
            //self.print(&winded_piece);
            self.wind_index += 1;

        
            if !self.is_collision(&winded_piece) {
                piece = winded_piece;
                
            }
            let descended_piece = piece.fall();
            //self.print(&descended_piece);
            
            if self.is_collision(&descended_piece)  {
                self.stop_piece(&piece);
                break;
            }
            else if piece.position.y == 0 {
                self.stop_piece(&piece);
                //self.print(&descended_piece);
                break;
            }
            else {
                piece = descended_piece;
                
            }
        }
        //println!("{}",self.wind_index - start_wind_index);
    }

    fn is_collision(&mut self, piece : &Piece) -> bool {
        let mut collide = false;
        for sub_piece in piece.parts_positions {
            collide = collide || match sub_piece {
                None => false,
                Some(coords) => self.get_cell(Coordinates{x : piece.position.x + coords.x, y : piece.position.y + coords.y})
            }
        }
        collide
    }

    fn stop_piece(&mut self, piece : &Piece) {
        for sub_piece in piece.parts_positions {
            match sub_piece {
                None => {},
                Some(coords) => self.set_cell(Coordinates{x : piece.position.x + coords.x, y : piece.position.y + coords.y}, true)
            }
        }
    }

    fn get_cell(&mut self, position : Coordinates) -> bool {
        if position.y >= self.lines.len() {
            self.extend_grid(position.y);
        }
        self.lines[position.y][position.x]
    }

    fn set_cell(&mut self, position : Coordinates, value : bool) {
        if position.y >= self.lines.len() {
            self.extend_grid(position.y);
        }
        self.lines[position.y][position.x] = value;
    }

    fn extend_grid(&mut self, height : usize) {
        while self.lines.len() <= height + 5 {
            self.lines.push([false; 7]);
        }
    }

    fn top_line_index(&self) -> Option<usize> {
        if self.lines.is_empty() {
            None
        }
        else {
            let mut index = self.lines.len() - 1;
            while index > 0 && self.lines[index].iter().all(|c| !(*c)) {
                index -= 1;
            }
            Some(index)
        }
    }

    pub fn print(&mut self, piece : &Piece) {
        println!("");
        println!("");
        println!("");
        for y in (0..self.lines.len()).rev() {
            for x in 0..7 {
                print!("{}", match self.get_cell(Coordinates { x, y }){
                    true => '#',
                    false => match self.is_in_piece(Coordinates { x, y }, piece) {
                        true => '@',
                        false => '.'
                    }
                } );
            }
            println!("");
        }
    }
    fn is_in_piece(&self, pos : Coordinates, piece : &Piece) -> bool{
        let mut collide = false;
        for sub_piece in piece.parts_positions {
            collide = collide || match sub_piece {
                None => false,
                Some(coords) => Coordinates{x : piece.position.x + coords.x, y : piece.position.y + coords.y} == pos
            }
        }
        collide
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right
}

pub fn day17_pt1() -> usize {
    let file = include_str!("../../inputs/day17.txt");
    //let file = "<<<<>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>";
    let mut all_pieces : [Piece; 5] = [Default::default(); 5];

    // Horizontal bar
    all_pieces[0].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 2, y : 0}), 
        Some(Coordinates{x : 3, y : 0}), 
        None];
    
    // Plus
    all_pieces[1].parts_positions = [
        Some(Coordinates{x : 0, y : 1}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 1, y : 1}), 
        Some(Coordinates{x : 2, y : 1}), 
        Some(Coordinates{x : 1, y : 2})];

    // Reverse L
    all_pieces[2].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 2, y : 0}), 
        Some(Coordinates{x : 2, y : 1}), 
        Some(Coordinates{x : 2, y : 2})];

    // Vertical bar
    all_pieces[3].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 0, y : 1}), 
        Some(Coordinates{x : 0, y : 2}), 
        Some(Coordinates{x : 0, y : 3}), 
        None];
        
    // Square
    all_pieces[4].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 0, y : 1}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 1, y : 1}), 
        None];

    let winds = file.chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Unexpected character")
    })
    .collect::<Vec<Direction>>();

    let mut grid = Grid {
        lines : vec!(),
        winds : winds,
        wind_index : 0
    };

    for i in 0..2022 {
        // if i == 16 {
        //     let x = 42;
        //     x + 1;
        // }
        grid.fall_piece(&all_pieces[i % 5]);
        //println!("{}", grid.top_line_index() + 1);
        //grid.print(&all_pieces[1]);
    }
    //grid.print(&all_pieces[1]);

    grid.top_line_index().unwrap() + 1

}

pub fn day17_pt2() -> usize {
    let file = include_str!("../../inputs/day17.txt");
    //let file = "<<<<>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>";
    let mut all_pieces : [Piece; 5] = [Default::default(); 5];

    // Horizontal bar
    all_pieces[0].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 2, y : 0}), 
        Some(Coordinates{x : 3, y : 0}), 
        None];
    
    // Plus
    all_pieces[1].parts_positions = [
        Some(Coordinates{x : 0, y : 1}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 1, y : 1}), 
        Some(Coordinates{x : 2, y : 1}), 
        Some(Coordinates{x : 1, y : 2})];

    // Reverse L
    all_pieces[2].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 2, y : 0}), 
        Some(Coordinates{x : 2, y : 1}), 
        Some(Coordinates{x : 2, y : 2})];

    // Vertical bar
    all_pieces[3].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 0, y : 1}), 
        Some(Coordinates{x : 0, y : 2}), 
        Some(Coordinates{x : 0, y : 3}), 
        None];
        
    // Square
    all_pieces[4].parts_positions = [
        Some(Coordinates{x : 0, y : 0}), 
        Some(Coordinates{x : 0, y : 1}), 
        Some(Coordinates{x : 1, y : 0}), 
        Some(Coordinates{x : 1, y : 1}), 
        None];

    let winds = file.chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Unexpected character")
    })
    .collect::<Vec<Direction>>();

    let mut grid = Grid {
        lines : vec!(),
        winds : winds,
        wind_index : 0
    };

    for i in 0..100000000 {
        // if i == 16 {
        //     let x = 42;
        //     x + 1;
        // }
        grid.fall_piece(&all_pieces[i % 5]);
        //println!("{}", grid.top_line_index() + 1);
        //grid.print(&all_pieces[1]);
    }
    //grid.print(&all_pieces[1]);

    grid.top_line_index().unwrap() + 1
}

#[cfg(test)]
mod tests {
    use crate::problems::day17::*;

    #[test]
    fn day17_pt1_test() {
        let result = day17_pt1();
        assert_eq!(result, 5176944);
    }

    #[test]
    fn day17_pt2_test() {
        let result = day17_pt2();
        assert_eq!(result, 13350458933732);
    }
}