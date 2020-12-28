use std::cmp::max;

fn generate_numbers() -> Vec<Vec<i32>> {
    let start = vec![1, 2, 5, 7, 3, 0];
    let end = vec![5, 7, 9, 3, 8, 1];

    let mut iter = vec![1, 2, 5, 7, 3, 0];
    let mut result: Vec<Vec<i32>> = Vec::new();

    loop {
        // from right to left, increase values
        for i in 0..6 {
            let i = 5 - i;
            iter[i] += 1;

            if iter[i] == 10 {
                iter[i] = 0;
            } else {
                break;
            }
        }

        // from left to right, assure each number is bigger or equal
        let mut mx = iter[0];

        for i in 0..6 {
            let nx = max(mx, iter[i]);

            iter[i] = nx;
            mx = nx;
        }

        // check if number is in bounds
        let mut still_valid = false;
        for i in 0..6 {
            if end[i] > iter[i] {
                still_valid = true;
                break;
            }
        }

        if !still_valid {
            break;
        }

        still_valid = false;

        // now check if any digit is in a sequence
        for i in 1..6 {
            if iter[i] == iter[i - 1] {
                still_valid = true;
                break;
            }
        }

        if still_valid {
            result.push(iter.clone());
        }
    }

    return result;
}

fn is_just_two(input: &Vec<i32>) -> bool {
    for &n in input {
        let mut count = 0;

        for i in 0..5 {
            let s = input[i];
            let s2 = input[i + 1];

            if s == s2 && s == n {
                count += 1;
            }
        }

        // there is exactly 1 case where the same number is repeated twice
        if count == 1 {
            return true;
        }
    }

    return false;
}

fn task2(numbers: Vec<Vec<i32>>) {
    let filtered = numbers
        .iter()
        .filter(|&x| is_just_two(x))
        .collect::<Vec<&Vec<i32>>>();
    println!("answer2: {}", filtered.len());
}

fn main() {
    let numbers = generate_numbers();
    println!("answer1: {}", numbers.len());
    task2(numbers);
}
