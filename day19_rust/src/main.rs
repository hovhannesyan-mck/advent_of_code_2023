use std::{fs, collections::HashMap, };
use rayon::prelude::*;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Rule {
    category: char,
    operator: char,
    value: i64,
    result: String,
}

impl Rule {
    fn new(category: char, operator: char, value: i64, result: String) -> Self {
        Self {
            category,
            operator,
            value,
            result,
        }
    }

    fn apply(&self, value: i64) -> bool {
        match self.operator {
            '>' => value > self.value,
            '<' => value < self.value,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    result: String,
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>, result: String) -> Self {
        Self { 
            name,
            rules, 
            result 
        }
    }

    fn apply(&self, part: &HashMap<char, i64>) -> String {
        for rule in &self.rules {
            if rule.apply(*part.get(&rule.category).unwrap()) {
                return rule.result.clone();
            }
        }
        self.result.clone()
    }
}

fn parse_workflow(
    line: &str
) -> (String, Workflow) {
    let split = line.split("{").collect::<Vec<&str>>();

    let name = split[0];

    let workflow_str =split[1].trim_end_matches('}');

    let rules_strs = workflow_str.split(",").collect::<Vec<&str>>();

    let rules = rules_strs.iter()
        .take(rules_strs.len() - 1)
        .map(|rule_str| {
            let split = rule_str.split(":").collect::<Vec<&str>>();
            
            let category = split[0].chars().nth(0).unwrap();
            let operator = split[0].chars().nth(1).unwrap();
            let value = split[0][2..].parse::<i64>().unwrap();

            let result = split[1].to_string();

            Rule::new(category, operator, value, result)
        })
        .collect();

    let result = rules_strs[rules_strs.len() - 1].trim().to_string();

    (name.to_owned(), Workflow::new(name.to_owned(), rules, result))
}

fn parse_part(
    line: &str
) -> HashMap<char, i64> {
    let trimmed = line.trim_matches(|c| c == '{' || c == '}');
    
    let part_strs = trimmed.split(",").collect::<Vec<&str>>();

    part_strs.iter()
        .map(|part_str| {
            let split = part_str.split("=").collect::<Vec<&str>>();
            
            let category = split[0].chars().nth(0).unwrap();
            let value = split[1].parse::<i64>().unwrap();

            (category, value)
        })
        .collect()
}

fn apply_workflows(
    workflows: &HashMap<String, Workflow>,
    part: &HashMap<char, i64>,
) -> i64 {
    let mut current_workflow = workflows.get("in").unwrap();

    loop {
        let result = current_workflow.apply(part);

        match result.as_str() {
            "R" => return 0,
            "A" => return part.values().sum(),
            str => current_workflow = workflows.get(str).unwrap(),
        }
    }
}

fn part_1(
    contents: &String,
) -> i64 {
    let (workflows_chunk, parts_chunk) = contents.split_at(contents.find("\n\n").unwrap());

    let workflows = workflows_chunk.lines()
        .map(|line| parse_workflow(line))
        .collect::<HashMap<String, Workflow>>();

    let parts = parts_chunk.lines()
        .skip(2)
        .map(|line| parse_part(line))
        .collect::<Vec<HashMap<char, i64>>>();

    parts.par_iter()
        .map(|part| apply_workflows(&workflows, part))
        .sum()
}

#[derive(Debug, Clone)]
struct Path {
    ranges: Vec<(String, Option<Rule>)>,
}

impl Path {
    fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    fn add_rule(&mut self, workflow_name: String, rule: Option<Rule>) {
        self.ranges.push((workflow_name, rule.clone()));
    }
}

fn accepted_dfs(
    workflows: &HashMap<String, Workflow>,
    workflow: &Workflow,
    path: &mut Path,
    paths: &mut Vec<Path>,
) {
    for rule in &workflow.rules {
        let mut path = path.clone();

        path.add_rule(workflow.name.clone(), Some(rule.clone()));

        if rule.result == "A" {
            paths.push(path);
        } else if rule.result != "R"{
            let next_workflow = workflows.get(&rule.result).unwrap();

            accepted_dfs(workflows, next_workflow, &mut path, paths);
        }
    }

    path.add_rule(workflow.name.clone(), None);

    if workflow.result == "A" {
        paths.push(path.clone());
    } else if workflow.result != "R" {
        let next_workflow = workflows.get(&workflow.result).unwrap();

        accepted_dfs(workflows, next_workflow, path, paths);
    }
}

fn calculate_sum_combinations(
    workflows: &HashMap<String, Workflow>,
    paths: &Vec<Path>,
) -> i64 {
    let mut sum = 0;

    for path in paths {
        let mut min_x = 1;
        let mut max_x = 4000;
        let mut min_m = 1;
        let mut max_m = 4000;
        let mut min_a = 1;
        let mut max_a = 4000;
        let mut min_s = 1;
        let mut max_s = 4000;

        for (workflow_name, path_rule) in &path.ranges {
            let workflow = workflows.get(workflow_name).unwrap();

            for rule in &workflow.rules {
                if let Some(path_rule) = path_rule {
                    if rule == path_rule {
                        match rule.category {
                        'x' => {
                            if rule.operator == '>' {
                                min_x = rule.value + 1;
                            } else {
                                max_x = rule.value - 1;
                            }
                        },
                        'm' => {
                            if rule.operator == '>' {
                                min_m = rule.value + 1;
                            } else {
                                max_m = rule.value - 1;
                            }
                        },
                        'a' => {
                            if rule.operator == '>' {
                                min_a = rule.value + 1;
                            } else {
                                max_a = rule.value - 1;
                            }
                        },
                        's' => {
                            if rule.operator == '>' {
                                min_s = rule.value + 1;
                            } else {
                                max_s = rule.value - 1;
                            }
                        },
                        _ => panic!("Invalid category"),
                    }
                        break;
                    }
                }
                
                match rule.category {
                    'x' => {
                        if rule.operator == '>' {
                            max_x = rule.value;
                        } else {
                            min_x = rule.value;
                        }
                    },
                    'm' => {
                        if rule.operator == '>' {
                            max_m = rule.value;
                        } else {
                            min_m = rule.value;
                        }
                    },
                    'a' => {
                        if rule.operator == '>' {
                            max_a = rule.value;
                        } else {
                            min_a = rule.value;
                        }
                    },
                    's' => {
                        if rule.operator == '>' {
                            max_s = rule.value;
                        } else {
                            min_s = rule.value;
                        }
                    },
                    _ => panic!("Invalid category"),
                }
            }
        }
        
        if min_x > max_x || min_m > max_m || min_a > max_a || min_s > max_s {
            continue;
        }

        let tmp =
            (max_x - min_x + 1) * (max_m - min_m + 1) * (max_a - min_a + 1) * (max_s - min_s + 1);
        
        sum += tmp;
    }

    sum
}

fn part_2(
    contents: &String,
) -> i64 {
    let (workflows_chunk, _) = contents.split_at(contents.find("\n\n").unwrap());

    let workflows = workflows_chunk.lines()
        .map(|line| parse_workflow(line))
        .collect::<HashMap<String, Workflow>>();

    let start_workflow = workflows.get("in").unwrap();

    let mut accepted_paths = Vec::new();
    let mut accepted_path = Path::new();
    
    accepted_dfs(&workflows, start_workflow, &mut accepted_path, &mut accepted_paths);

    calculate_sum_combinations(&workflows, &accepted_paths)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 19114);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 167409079868000);
    }
}
