use std::{collections::HashSet};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Cube {
    x : i32,
    y : i32,
    z : i32
}

impl Cube {
    pub fn new(line : &str) -> Cube {
        let mut splitted = line.split(',');
        let x = splitted.next().unwrap().parse::<i32>().unwrap();
        let y = splitted.next().unwrap().parse::<i32>().unwrap();
        let z = splitted.next().unwrap().parse::<i32>().unwrap();
        Cube { x, y, z }
    }

    pub fn get_adjacent_faces(&self) -> [Cube; 6] {
        [Cube {x:self.x-1, y:self.y, z:self.z},
        Cube {x:self.x+1, y:self.y, z:self.z},
        Cube {x:self.x, y:self.y-1, z:self.z},
        Cube {x:self.x, y:self.y+1, z:self.z},
        Cube {x:self.x, y:self.y, z:self.z-1},
        Cube {x:self.x, y:self.y, z:self.z+1}]
    }

    pub fn is_in_bounds(&self, bounds : &Bounds) -> bool {
        self.x >= bounds.min_x &&
            self.x <= bounds.max_x &&
            self.y >= bounds.min_y && 
            self.y <= bounds.max_y &&
            self.z >= bounds.min_z &&
            self.z <= bounds.max_z
    }
}

struct Bounds {
    min_x : i32, 
    max_x : i32,
    min_y : i32,
    max_y : i32,
    min_z : i32, 
    max_z : i32
}

fn generate_air_cubes(lava_cubes : &HashSet<Cube>, bounds : &Bounds) -> HashSet<Cube> {
    let mut result = HashSet::<Cube>::new();
    let mut stack : Vec<Cube> = vec!();

    stack.push(Cube{x : bounds.min_x, y : bounds.min_y, z : bounds.min_z});

    while let Some(cube) = stack.pop() {
        if !result.contains(&cube) && !lava_cubes.contains(&cube) && cube.is_in_bounds(bounds) {
            let mut adjacents = cube.get_adjacent_faces().iter().cloned().collect::<Vec<Cube>>();
            stack.append(&mut adjacents);
            result.insert(cube);
        }
    }
    result
}

pub fn day18_pt1() -> usize {
    let file = include_str!("../../inputs/day18.txt");
    
    let cubes : HashSet<Cube> = file.split('\n')
        .map(|l| Cube::new(l.trim()))
        .collect();
        
    cubes
        .iter()
        .map(|cube| cube.get_adjacent_faces())
        .flatten()
        .filter(|possible_cube| !cubes.contains(possible_cube))
        .count()
}

pub fn day18_pt2() -> usize {
    let file = include_str!("../../inputs/day18.txt");
    
    let lava_cubes : HashSet<Cube> = file.split('\n')
        .map(|l| Cube::new(l.trim()))
        .collect();

    let min_x = lava_cubes.iter().map(|c| c.x).min().unwrap() - 1;
    let max_x = lava_cubes.iter().map(|c| c.x).max().unwrap() + 1;

    let min_y = lava_cubes.iter().map(|c| c.y).min().unwrap() - 1;
    let max_y = lava_cubes.iter().map(|c| c.y).max().unwrap() + 1;

    let min_z = lava_cubes.iter().map(|c| c.z).min().unwrap() - 1;
    let max_z = lava_cubes.iter().map(|c| c.z).max().unwrap() + 1;
    
    let bounds =  Bounds {
        min_x, max_x, min_y, max_y, min_z, max_z
    };

    let air_cubes = generate_air_cubes(&lava_cubes, &bounds);

    lava_cubes
        .iter()
        .map(|cube| cube.get_adjacent_faces())
        .flatten()
        .filter(|possible_cube| air_cubes.contains(possible_cube))
        .count()
    
}

#[cfg(test)]
mod tests {
    use crate::problems::day18::*;

    #[test]
    fn day18_pt1_test() {
        let result = day18_pt1();
        assert_eq!(result, 4364);
    }

    #[test]
    fn day18_pt2_test() {
        let result = day18_pt2();
        assert_eq!(result, 2508);
    }
}