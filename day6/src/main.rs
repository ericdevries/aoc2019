use std::collections::HashMap;
use std::fs;

fn find_root(child: &str, tree: &HashMap<&str, &str>) -> i32 {
    match tree.get(child) {
        Some(x) => 1 + find_root(x, &tree),
        None => 0
    }
}

fn task1(lines: &Vec<&str>) {
    let mut refs: HashMap<&str, &str> = HashMap::new();

    for line in lines {
        let v: Vec<&str> = line.split(")").collect();
        let v1 = v[0];
        let v2 = v[1];
        
        // insert child -> parent
        refs.insert(v2, v1);
    }
   
    let answer: i32 = refs.keys().map(|k| find_root(k, &refs)).sum();
    println!("answer1: {}", answer);
}

fn find_path<'a>(child: &'a str, tree: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    let mut it = child;

    loop {
        match tree.get(it) {
            Some(x) => {
                it = x;
                res.push(x);
            },
            None => break
        }
    }

    return res
}

fn task2(lines: &Vec<&str>) {
    let mut refs: HashMap<&str, &str> = HashMap::new();

    for line in lines {
        let v: Vec<&str> = line.split(")").collect();
        let v1 = v[0];
        let v2 = v[1];
        
        // insert child -> parent
        refs.insert(v2, v1);
    }
    
    let path1 = find_path("YOU", &refs);
    let path2 = find_path("SAN", &refs);

    for (j, p) in path1.iter().enumerate() {
        match path2.iter().position(|x| x == p) {
            Some(x) => {
                println!("answer2: {}", x + j);
                break;
            }
            None => {}
        }
    }
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").map(|s| s.trim()).filter(|s| s.len() > 0).collect();

    println!("lines: {:?}", lines);
    task1(&lines);
    task2(&lines);
}
