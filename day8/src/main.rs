use std::fs;

fn create_image(width: i32, height: i32, input: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::with_capacity((width * height) as usize);

    for i in 0..(width * height) {
        result.push(Vec::new());
    }

    for (i, c) in input.chars().enumerate() {
        let it = (i as i32) % (width * height);
        result[it as usize].push(c);
    }

    return result;
}

fn count_zeros(input: &Vec<char>, cc: char) -> usize {
    return input.iter().filter(|&&c| c == cc).collect::<Vec<_>>().len();
}

fn count_layer(input: &Vec<Vec<char>>) -> Vec<(usize, usize, usize, usize)> {
    let layers = input[0].len();
    let mut result: Vec<(usize, usize, usize, usize)> = Vec::new();

    for i in 0..layers {
        let vec = input.iter().map(|x| x[i]).collect();

        result.push((
            i,
            count_zeros(&vec, '0'),
            count_zeros(&vec, '1'),
            count_zeros(&vec, '2'),
        ));
    }

    return result;
}

fn stack_image(input: &Vec<Vec<char>>) -> Vec<char> {
    let mut result: Vec<char> = Vec::with_capacity(input.len());
    
    for x in input {
        for cx in x {
            if *cx != '2' {
                result.push(match *cx {
                    '1' => '*',
                    _ => '%'
                });

                break;
            }
        }
    }
    
    return result;
}

fn task1(input: &str) {
    let width = 25;
    let height = 6;

    let img = create_image(width, height, input);
    let mut counts = count_layer(&img);
    counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    let res = counts[0];
    println!("answer1: {}", res.2 * res.3);
}

fn task2(input: &str) {
    let width = 25;
    let height = 6;
    let img = create_image(width, height, input);
    let stacked = stack_image(&img);
    
    for j in 0..height {
        for i in 0..width {
            print!("{},", stacked[(j*width + i) as usize]);
        }
        println!();
    }
 }

fn main() {
    let contents = match fs::read_to_string("data.txt") {
        Ok(x) => String::from(x.trim()),
        Err(e) => String::from(""),
    };

    // task1(&contents);
    task2(&contents);
}
