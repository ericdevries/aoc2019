use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Unit {
    amount: i64,
    name: String,
    requirements: Vec<(String, i64)>,
}

fn parse_requirement(input: &str) -> (String, i64) {
    let re = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    let cap = re.captures_iter(input).next().unwrap();

    let name = &cap[2];
    let amount = cap[1].parse::<i64>().unwrap();

    return (name.to_string(), amount);
}

fn parse(input: &str) -> Unit {
    let parts: Vec<&str> = input.split(" => ").collect();
    let (name, amount) = parse_requirement(parts[1]);
    let reqs: Vec<(String, i64)> = parts[0].split(", ").map(|s| parse_requirement(s)).collect();

    return Unit {
        amount: amount,
        name: name,
        requirements: reqs,
    };
}

fn find_ore(
    node: &Unit,
    input: &HashMap<String, Unit>,
) -> i64 {
    let mut result: i64 = 0;

    for (_, unit) in input {
        for (req, count) in &unit.requirements {
            if req == &node.name && unit.name == "FUEL" {
                result += unit.amount * *count;
            } else if req == &node.name {
                let c = find_ore(unit, input);
                let mut here = c / unit.amount;
                //println!("dbg: {}, {}, {}", c, here, unit.amount);

                if c % unit.amount != 0 {
                    here += 1;
                }

                //println!("to generate {} {}, we need {} {}", node.amount, node.name, here, unit.name);

                result += here * count;
            }
        }
    }

    return result;
}

fn get_ore(input: &HashMap<String, Unit>) -> i64 {
    let mut orecount: i64 = 0;

    for (_, unit) in input {
        for (req, count) in &unit.requirements {
            if req == "ORE" {
                let ore = find_ore(&unit, &input);
                // println!("name: {} = {} ({})", unit.name, ore, unit.amount);

                let mut c = ore / unit.amount;

                if ore % unit.amount != 0 {
                    c += 1;
                }
/*
                println!(
                    "result: {} ({}) = {} ({})",
                    unit.name,
                    unit.amount,
                    c * count,
                    ore
                );
                */

                orecount += c * count;
            }
        }
    }

    return orecount;
}

fn task1(input: &HashMap<String, Unit>) {
    let mut data = input.clone();
    let orecount = get_ore(input);

    println!("answer1: {:?}", orecount);
    
    let mut low: i64 = 1;
    let mut high: i64 = 1000000000000;
    let target: i64 = 1000000000000;
    let mut answer: i64 = 0;

    while low <= high {
        let mid = (high - low) / 2 + low;
        data.entry(String::from("FUEL"))
            .and_modify(|e| e.amount = mid);
        
        let val = get_ore(&data);
        answer = mid;

        if val < target {
            low = mid + 1;
        } else if val > target {
            high = mid - 1;
        }


        // println!("value for ore {}: {}", mid, val);
    }
    
    println!("answer2: {}", answer);
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();

    let input: Vec<Unit> = contents
        .split("\n")
        .filter(|s| s.trim().len() > 0)
        .map(parse)
        .collect();
    let mut map: HashMap<String, Unit> = HashMap::new();

    for i in input {
        map.insert(String::from(&i.name), i);
    }

    task1(&map);
}
