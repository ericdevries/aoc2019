use num::integer::{gcd, lcm};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use primes::factors_uniq;

type Three = (i32, i32, i32);
type Four = (i32, i32, i32, i32);

fn parse_num(s: &str) -> i32 {
    let s = s.replace("<", "");
    let s = s.replace(">", "");
    s.split("=").collect::<Vec<&str>>()[1].parse().unwrap()
}

fn process_row(s: &str) -> Three {
    let parts: Vec<&str> = s.split(", ").collect();
    let x: i32 = parse_num(parts[0]);
    let y: i32 = parse_num(parts[1]);
    let z: i32 = parse_num(parts[2]);

    return (x, y, z);
}

fn rel_vel(x: i32, x2: i32) -> i32 {
    if x > x2 {
        return -1;
    } else if x < x2 {
        return 1;
    }
    return 0;
}

fn sum(input: &Three) -> i32 {
    return input.0.abs() + input.1.abs() + input.2.abs();
}

fn calculate_velocities(
    positions: &Vec<Three>,
    velocities: &Vec<Three>,
) -> Vec<Three> {
    let mut new_velocities = velocities.clone();

    for (i, x) in (&positions).iter().enumerate() {
        let mut velocity = velocities[i];

        for (j, y) in positions.iter().enumerate() {
            if i == j {
                continue;
            }

            velocity = (
                velocity.0 + rel_vel(x.0, y.0),
                velocity.1 + rel_vel(x.1, y.1),
                velocity.2 + rel_vel(x.2, y.2),
            );
        }

        new_velocities[i] = velocity;
    }

    return new_velocities;
}

fn calculate_positions(
    positions: &Vec<Three>,
    velocities: &Vec<Three>,
) -> Vec<Three> {
    let mut data = positions.clone();

    for (i, x) in velocities.iter().enumerate() {
        data[i] = (data[i].0 + x.0, data[i].1 + x.1, data[i].2 + x.2);
    }

    return data;
}

fn task1(input: &Vec<Three>) {
    let mut data = input.clone();
    let mut velocities: Vec<Three> = Vec::new();

    for _ in &data {
        velocities.push((0, 0, 0));
    }

    for _ in 0..1000 {
        velocities = calculate_velocities(&data, &velocities);
        data = calculate_positions(&data, &velocities);
    }

    let total: i32 = data
        .iter()
        .zip(velocities)
        .map(|(p, v)| sum(p) * sum(&v))
        .sum();

    println!("answer1: {}", total);
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let positions: Vec<(i32, i32, i32)> = contents
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(process_row)
        .collect();

    task1(&positions);
    println!("Positions: {:?}", positions);
}
