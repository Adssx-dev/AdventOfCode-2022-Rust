use std::{collections::{HashMap, HashSet}, cmp::max};

#[derive(Clone, Debug)]
struct Valve {
    flow_rate : i32,
    neighbors : Vec<String>
}

struct ValveNetwork {
    valves :  HashMap<String, Valve>,
    all_valves_values : Vec<i32>,
}

impl ValveNetwork {
    pub fn new(file : &str) -> ValveNetwork {
        let valves_description = file.split("\n")
            .map(|line| line.trim()
                .split(" "))
                .map(|mut lines| (lines.nth(1).unwrap(), lines.nth(2).unwrap(), lines.skip(4).collect::<Vec<&str>>()));
        
        let mut result = ValveNetwork {
            valves : HashMap::new(),
            all_valves_values : vec!()
        };

        for v in valves_description {
            let default_valve = Valve {
                flow_rate : 0,
                neighbors : vec!()
            };
            
            let id = v.0;
            let flow = v.1.split("=").nth(1).unwrap().replace(";", "").parse::<i32>().unwrap();
            let neighbors = v.2.iter().map(|id| id.replace(",", "")).collect::<Vec<String>>();
            
            // Not needed
            // for neighbor in neighbors.clone() {
            //     result.valves.entry(neighbor.to_owned()).or_insert(default_valve.clone()).neighbors.push(id.to_owned());
            // }

            let valve = result.valves.entry(id.to_owned()).or_insert(default_valve);
            valve.flow_rate = flow;
            valve.neighbors = neighbors;
        }

        result.all_valves_values = result.valves.iter()
            .map(|v| v.1.flow_rate)
            .filter(|f| *f > 0)
            .collect::<Vec<i32>>();
        result.all_valves_values.sort_unstable_by(|a, b| b.cmp(a));


        result
    }

    pub fn calculate_released_pressure(&self, actions : &[Action], time_limit : i32) -> i32 {
        let mut remaining_time = time_limit;
        let mut released_pressure = 0;

        for action in actions {
            remaining_time -= 1;
            if remaining_time < 0 {
                panic!("Too many actions");
            }
            released_pressure += match action {
                Action::Open(v) => self.valves[v].flow_rate * remaining_time,
                Action::Move(_) => 0
            }
        }
        released_pressure
    }

    pub fn get_possible_actions(&self, actions : Option<&[Action]>) -> Vec<Action> {
        let current_position = match actions {
            None => "AA",
            Some(acts) => match acts.last().unwrap()
            {
                Action::Move(id) => id,
                Action::Open(id) => id
            }
        };

        let mut possible_actions  = self.valves[current_position]
            .neighbors
            .iter()
            .map(|neighbor| Action::Move(neighbor.clone()))
            .collect::<Vec<Action>>();
        
        if ! self.is_valve_open(current_position, actions) && self.valves[current_position].flow_rate > 0 {
            possible_actions.push(Action::Open(current_position.to_owned()));
        }

        possible_actions
    }

    fn is_valve_open(&self, valve_id : &str, actions : Option<&[Action]>) -> bool {
        if let Some(acts) = actions {
            acts.iter().filter_map(|action| match action {
                Action::Open(id) => Some(id),
                Action::Move(_) => None
            })
            .any(|id| valve_id == id)
        }
        else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Action {
    Move(String),
    Open(String)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SolutionCandidate {
    actions : Vec<Action>,
    released_pressure : i32
}

struct Solver {
    network : ValveNetwork,
    candidates : HashSet<SolutionCandidate>,
    best_solution : i32
}

impl Solver {

    pub fn solve(&mut self) -> i32 {
        let starting_actions = self.network.get_possible_actions(None);

        for action in starting_actions {
            let actions = vec!(action);
            let released = self.network.calculate_released_pressure(&actions, 30);
            let first_candidate = SolutionCandidate {
                actions,
                released_pressure : released
            };
            if !self.candidates.contains(&first_candidate) {
                self.candidates.insert(first_candidate);
            }
        }

        while let Some(candidate) = self.get_best_candidate() {
            self.candidates.remove(&candidate);
            let possible_actions = self.network.get_possible_actions(Some(&candidate.actions));

            for action in possible_actions {
                let mut actions = candidate.actions.clone();
                if actions.len() < 30 {
                    actions.push(action);
                    let released = self.network.calculate_released_pressure(&actions, 30);
                    let new_candidate = SolutionCandidate {
                        actions,
                        released_pressure : released
                    };
                    if self.best_solution <  released {
                        self.best_solution = released;
                        println!("New best : {}", released);
                        self.filter_all_solution();
                    }
                    if self.is_solution_interesting(&candidate) &&  !self.candidates.contains(&new_candidate) {
                        self.candidates.insert(new_candidate);
                    }
                }
            }
        }
        self.best_solution
    }
    
    fn filter_all_solution(&mut self) {
        self.candidates = self.candidates.iter()
            .filter(|candidate| candidate.actions.len() < 30)
            .filter(|candidate| self.is_solution_interesting(candidate))
            .map(|candidate| candidate.clone())
            .collect::<HashSet<SolutionCandidate>>();
    }

    fn is_solution_interesting(&self, candidate : &SolutionCandidate) -> bool {
        let candidate_pressure = candidate.released_pressure;
        let mut candidate_max_pressure = candidate_pressure;
        let remaining_steps = 30 - candidate.actions.len();
        let mut i : usize = 0;
        while i * 4 < remaining_steps && i < self.network.all_valves_values.len() {
            candidate_max_pressure += self.network.all_valves_values[i] * (remaining_steps - i - 1) as i32;
            i+=1;
        }
        candidate_max_pressure > self.best_solution
    }

    fn get_best_candidate(&self) -> Option<SolutionCandidate> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.actions.len() < 30)
            .max_by(|e1, e2| e1.released_pressure.cmp(&e2.released_pressure))
            .map(|c| c.clone())
    }
}

pub fn day16_pt1() -> usize {
    let file = include_str!("../../inputs/day16.txt");

    let valve_network = ValveNetwork::new(file);

    let mut solver = Solver{
        network: valve_network,
        candidates : HashSet::new(),
        best_solution : -1
    };

    println!("{:?}",solver.solve() );
    //println!("{:?}", valve_network.valves["AA"]);
    //println!("{:?}", valve_network.get_possible_actions(None));
    0

}

pub fn day16_pt2() -> i64 {
    let file = include_str!("../../inputs/day16.txt");

    0
}

#[cfg(test)]
mod tests {
    use crate::problems::day16::*;

    #[test]
    fn day16_pt1_test() {
        let result = day16_pt1();
        assert_eq!(result, 5176944);
    }

    #[test]
    fn day16_pt2_test() {
        let result = day16_pt2();
        assert_eq!(result, 13350458933732);
    }
}