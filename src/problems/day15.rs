use std::cmp::{min, max};

#[derive(PartialEq, Eq)]
struct Coordinates {
    x : i32,
    y : i32
}

impl Coordinates {
    pub fn distance(&self, other : &Coordinates) -> i32 {
        (self.x - other.x).abs() +
            (self.y - other.y).abs()
    }
}

struct Sensor {
    position : Coordinates,
    closest_beacon : Coordinates
}

impl Sensor {
    pub fn new(line : &str) -> Sensor {
        let mut splitted = line.split(' ');

        splitted.nth(1); // Skip 2 elements
        let x_sensor_str = splitted.next().unwrap().trim();
        let y_sensor_str = splitted.next().unwrap().trim();
        splitted.nth(3); // Skip 4 elements
        let x_beacon_str = splitted.next().unwrap().trim();
        let y_beacon_str = splitted.next().unwrap().trim();

        Sensor {
            position : Coordinates { 
                x: x_sensor_str[2..x_sensor_str.len() - 1].parse::<i32>().unwrap(), 
                y: y_sensor_str[2..y_sensor_str.len() - 1].parse::<i32>().unwrap(), 
            },
            closest_beacon : Coordinates { 
                x: x_beacon_str[2..x_beacon_str.len() - 1].parse::<i32>().unwrap(), 
                y: y_beacon_str[2..y_beacon_str.len()].parse::<i32>().unwrap() 
            }
        }
    }

    pub fn beacon_distance(&self) -> i32 {
        self.position.distance(&self.closest_beacon)
    }

    pub fn total_range_x(&self) -> (i32, i32) {
        let dist_x = self.beacon_distance();
        let range_1 = self.position.x - dist_x;
        let range_2 = self.position.x + dist_x;
        (min(range_1, range_2), max(range_1, range_2))
    }
}


pub fn day15_pt1() -> usize {
    let file = include_str!("../../inputs/day15.txt");

    let sensors : Vec<Sensor> = file.split('\n')
        .map(|line| Sensor::new(line))
        .collect();
    
    let min_x = sensors.iter().map(|sensor| sensor.total_range_x().0)
        .min()
        .unwrap();

    let max_x = sensors.iter().map(|sensor| sensor.total_range_x().1)
        .max()
        .unwrap();

    let interest_line = 2000000;

    (min_x..=max_x)
        .map(|x| Coordinates{x, y : interest_line})
        .filter(|point| sensors.iter()
           .any(|sensor| sensor.position.distance(point) <= sensor.beacon_distance()) &&
            ! sensors.iter().any(|sensor| sensor.closest_beacon == *point))
        .count()

}

pub fn day15_pt2() -> u32 {
    let file = include_str!("../../inputs/day15.txt");

    0
}

#[cfg(test)]
mod tests {
    use crate::problems::day15::*;

    #[test]
    fn day15_pt1_test() {
        let result = day15_pt1();
        assert_eq!(result, 755);
    }

    #[test]
    fn day15_pt2_test() {
        let result = day15_pt2();
        assert_eq!(result, 29805);
    }
}
