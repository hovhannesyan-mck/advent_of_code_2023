use std::fs;
use std::collections::{HashMap, VecDeque};
use num_integer::lcm;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Module {
    Broadcaster(BroadcasterData),
    FlipFlop(FlipFlopData),
    Conjunction(ConjunctionData),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct BroadcasterData {
    send_to: Vec<String>,
}

impl BroadcasterData {
    fn new(send_to: Vec<String>) -> Self {
        Self { 
            send_to 
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FlipFlopData {
    name: String,
    state: bool,
    send_to: Vec<String>,
}

impl FlipFlopData {
    fn new(name: String, send_to: Vec<String>) -> Self {
        let state = false;

        Self { 
            name, 
            state, 
            send_to 
        }
    }

    fn switch_state(&mut self) {
        self.state = !self.state;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ConjunctionData {
    name: String,
    state: HashMap<String, Pulse>,
    send_to: Vec<String>,
}

impl ConjunctionData {
    fn new(name: String, send_to: Vec<String>) -> Self {
        let state = HashMap::new();

        Self { 
            name, 
            state, 
            send_to 
        }
    }

    fn add_states(&mut self, modules: &Vec<Module>) {
        modules.iter()
            .for_each(|module| {
                match module {
                    Module::FlipFlop(data) => {
                        if data.send_to.contains(&self.name) {
                            self.state.insert(data.name.clone(), Pulse::Low);
                        }
                    },
                    Module::Conjunction(data) => {
                        if data.send_to.contains(&self.name) {
                            self.state.insert(data.name.clone(), Pulse::Low);
                        }
                    },
                    _ => {},
                }
            });
    }

    fn update_state(&mut self, name: &String, pulse: Pulse) {
        if let Some(state) = self.state.get_mut(name) {
            *state = pulse.clone();
        }
    }

    fn get_pulse(&self) -> Pulse {
        if self.state.values()
            .all(|&pulse| pulse == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            }
    }
}

fn parse_data(
    contents: &String,
) -> HashMap<String, Module> {
    let mut modules_map = HashMap::new();

    contents.lines()
        .for_each(|line| {
            let split = line.split(" -> ").collect::<Vec<_>>();

            let send_to = split[1].split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();

            match split[0].chars().nth(0).unwrap() {
                '%' => {
                    let name = split[0].chars().skip(1).collect::<String>();
                    modules_map.insert(name.clone(), Module::FlipFlop(FlipFlopData::new(name, send_to)));
                }
                '&' => {
                    let name = split[0].chars().skip(1).collect::<String>();
                    modules_map.insert(name.clone(), Module::Conjunction(ConjunctionData::new(name, send_to)));
                }
                _ => {
                    modules_map.insert(split[0].to_owned(), Module::Broadcaster(BroadcasterData::new(send_to)));
                }
            }
        });

    let modules = modules_map.values().cloned().collect::<Vec<_>>();

    modules_map.values_mut()
        .for_each(|module| {
            match module {
                Module::Conjunction(data) => {
                    data.add_states(&modules);
                },
                _ => {},
            }
        });

    modules_map
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueNode {
    module_name: String,
    pulse: Pulse,
    sender: String,
}

impl QueueNode {
    fn new(module_name: String, pulse: Pulse, sender: String) -> Self {
        Self {
            module_name,
            pulse,
            sender,
        }
    }
}

fn process_signal(
    modules_map: &mut HashMap<String, Module>,
    queue: &mut VecDeque<QueueNode>,
    number_low: &mut i64,
    number_high: &mut i64,
) {
    let node = match queue.pop_front() {
        Some(node) => node,
        None => return, 
    };

    match node.pulse {
        Pulse::Low => *number_low += 1,
        Pulse::High => *number_high += 1,
    }

    if let Some(module) = modules_map.get_mut(&node.module_name) {
        match module {
            Module::Broadcaster(data) => {
                data.send_to.iter()
                    .for_each(|module_name| {
                        queue.push_back(QueueNode::new(module_name.clone(), node.pulse, node.module_name.clone()));
                    });
            },
            Module::FlipFlop(data) => {
                if node.pulse != Pulse::High {
                    data.switch_state();
    
                    let pulse = if data.state {
                        Pulse::High
                    } else {
                        Pulse::Low
                    };
        
                    data.send_to.iter()
                        .for_each(|module_name| {
                            queue.push_back(QueueNode::new(module_name.clone(), pulse, node.module_name.clone()));
                        });
                }                
            },
            Module::Conjunction(data) => {
                data.update_state(&node.sender, node.pulse);

                let pulse = data.get_pulse();

                data.send_to.iter()
                    .for_each(|module_name| {
                        queue.push_back(QueueNode::new(module_name.clone(), pulse, node.module_name.clone()));
                    });
            },
        }
    }

    process_signal(modules_map, queue, number_low, number_high)
}

fn part_1(
    contents: &String,
) -> i64 {
    let mut modules_map = parse_data(contents);
    let mut number_low = 0;
    let mut number_high = 0;
    let mut queue = VecDeque::new();
    
    for _ in 0..1000 {
        queue.push_back(QueueNode::new("broadcaster".to_owned(), Pulse::Low, "broadcaster".to_owned()));
        process_signal(&mut modules_map, &mut queue, &mut number_low, &mut number_high);
    }

    number_low * number_high
}

fn process_signal_part_2(
    modules_map: &mut HashMap<String, Module>,
    queue: &mut VecDeque<QueueNode>,
    counter: i64,
    cycle_lengths: &mut HashMap<String, i64>,
    seen: &mut HashMap<String, bool>,
    to_rx_mod: String,
) {
    let node = match queue.pop_front() {
        Some(node) => node,
        None => return, 
    };

    if node.module_name == to_rx_mod {
        if let Some(seen) = seen.get_mut(&node.sender) {
            if node.pulse == Pulse::High {
                *seen = true;
                cycle_lengths.insert(node.sender.clone(), counter);
            }
        }
    } 

    if let Some(module) = modules_map.get_mut(&node.module_name) {
        match module {
            Module::Broadcaster(data) => {
                data.send_to.iter()
                    .for_each(|module_name| {
                        queue.push_back(QueueNode::new(module_name.clone(), node.pulse, node.module_name.clone()));
                    });
            },
            Module::FlipFlop(data) => {
                if node.pulse != Pulse::High {
                    data.switch_state();
    
                    let pulse = if data.state {
                        Pulse::High
                    } else {
                        Pulse::Low
                    };
        
                    data.send_to.iter()
                        .for_each(|module_name| {
                            queue.push_back(QueueNode::new(module_name.clone(), pulse, node.module_name.clone()));
                        });
                }                
            },
            Module::Conjunction(data) => {
                data.update_state(&node.sender, node.pulse);

                let pulse = data.get_pulse();

                data.send_to.iter()
                    .for_each(|module_name| {
                        queue.push_back(QueueNode::new(module_name.clone(), pulse, node.module_name.clone()));
                    });
            },
        }
    }

    process_signal_part_2(modules_map, queue, counter, cycle_lengths, seen, to_rx_mod)
}

fn part_2(
    contents: &String,
) -> i64 {
    let mut modules_map = parse_data(contents);
    let mut counter = 0;
    let mut queue = VecDeque::new();

    let mut cycle_lengths = HashMap::new();
    let mut seen = HashMap::new();

    let to_rx_mod = modules_map.values()
        .filter_map(|module| {
            match module {
                Module::Conjunction(data) 
                    if data.send_to.iter().any(|module_name| module_name == "rx") 
                        => Some(data.clone()),
                _ => None,
            }
        })
        .next()
        .unwrap();

    to_rx_mod.state.keys()
        .for_each(|module_name,| {
            seen.insert(module_name.clone(), false);
        });

    
    while seen.values().any(|&value| !value) {
        counter += 1;
        queue.push_back(QueueNode::new("broadcaster".to_owned(), Pulse::Low, "broadcaster".to_owned()));
        process_signal_part_2(&mut modules_map, &mut queue, counter, &mut cycle_lengths, &mut seen, to_rx_mod.name.clone());
    }

    cycle_lengths.values()
        .fold(1, |acc, &cycle_length| lcm(acc, cycle_length))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_input_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 32000000);
    }
    
    #[test]
    fn test_part_1_input_2() {
        let file_path = "test_input_2.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 11687500);
    }
}
