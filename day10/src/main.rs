use std::collections::HashMap;
use std::fs;

fn get_distance_map(ox: i32, oy: i32, input: &Vec<Vec<char>>) -> HashMap<i32, Vec<(i32, i32)>> {
    let mut seen: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for (j, y) in input.iter().enumerate() {
        for (i, x) in y.iter().enumerate() {
            let i = i as i32;
            let j = j as i32;

            if ox == i && oy == j {
                continue;
            }

            if *x == '#' {
                let dx = (i - ox) as f32;
                let dy = (j - oy) as f32;
                let angle = dx.atan2(dy);
                let iangle: i32 = (angle * 1000000.0) as i32;

                let lst = seen.entry(iangle).or_insert(Vec::new());
                lst.push((i, j));
            }
        }
    }

    return seen;
}

fn find_distances(ox: i32, oy: i32, input: &Vec<Vec<char>>) -> usize {
    let seen = get_distance_map(ox, oy, &input);
    return seen.keys().len();
}

fn task1(input: &Vec<Vec<char>>) {
    let mut lst: Vec<usize> = Vec::new();

    for (j, y) in input.iter().enumerate() {
        for (i, x) in y.iter().enumerate() {
            let i = i as i32;
            let j = j as i32;

            if *x == '#' {
                let distances = find_distances(i, j, &input);
                lst.push(distances);
            }
        }
    }

    lst.sort();
    println!("answer1: {}", lst.last().unwrap());
}

fn distance(x: i32, y: i32, ox: i32, oy: i32) -> i32 {
    return (x - ox).abs() + (y - oy).abs();
}

fn sorted_by_distance(x: i32, y: i32, input: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result = input.clone();
    result.sort_by(|a, b| {
        distance(x, y, b.0, b.1)
            .partial_cmp(&distance(x, y, a.0, a.1))
            .unwrap()
    });

    return result;
}
fn task2(input: &Vec<Vec<char>>) {
    let mut lst: Vec<(usize, i32, i32)> = Vec::new();

    for (j, y) in input.iter().enumerate() {
        for (i, x) in y.iter().enumerate() {
            let i = i as i32;
            let j = j as i32;

            if *x == '#' {
                let distances = find_distances(i, j, &input);
                lst.push((distances, i, j));
            }
        }
    }

    lst.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let winner = lst.last().unwrap();

    // println!("winner: {:?}", winner);
    let chart = get_distance_map(winner.1, winner.2, &input);
    let mut sortedlst: Vec<(i32, Vec<(i32, i32)>)> = chart
        .iter()
        .map(|(k, v)| (*k, sorted_by_distance(winner.1, winner.2, &v)))
        .collect();

    sortedlst.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let mut start = sortedlst.iter().position(|x| x.0 == 3141592).unwrap();
    let mut counter = 0;
    let length = sortedlst.len();
    
    // keep removing items until we hit 200
    loop {
        if start >= length {
            start = 0;
        }

        let item = &sortedlst[start];

        if item.1.len() > 0 {
            let popped = sortedlst[start].1.pop().unwrap();
            counter += 1;
        
            // println!("{} -> popped: {:?}", counter, popped);
            if counter >= 200 {
                println!("answer2: {}", popped.0 * 100 + popped.1);
                break;
            }
        }

        start += 1;
    }
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let contents: Vec<Vec<char>> = contents
        .split("\n")
        .map(|x| x.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();

    task1(&contents);
    task2(&contents);
}
