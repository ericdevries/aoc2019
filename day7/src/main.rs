use std::fs;
use std::cmp::max;
use permutohedron::heap_recursive;

struct Buffer {
    iter: usize,
    first: bool,
    amp: i32,
    buf: Vec<i32>,
}

impl Buffer {
    fn fetch_op(&mut self) -> i32 {
        if self.iter >= self.buf.len() {
            return 99;
        }

        let val = self.buf[self.iter];
        self.iter += 1;
        return val;
    }

    fn fetch_data(&mut self, mode: i32) -> i32 {
        let op = self.fetch_op();
        let val = match mode {
            0 => self.buf[op as usize],
            1 => op,
            _ => op,
        };

        return val;
    }

    fn execute(&mut self, input_value: i32) -> i32 {
        loop {
            let op_raw = self.fetch_op();
            let op = op_raw % 100;

            let m1 = op_raw % 1000 / 100;
            let m2 = op_raw % 10000 / 1000;

            if op == 1 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                let pos = self.fetch_op();
                self.buf[pos as usize] = o1 + o2;
            } else if op == 2 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                let pos = self.fetch_op();
                self.buf[pos as usize] = o1 * o2;
            } else if op == 3 {
                let o1 = self.fetch_op();
                self.buf[o1 as usize] = match self.first {
                    true => self.amp,
                    false => input_value
                };
                self.first = false;
            } else if op == 4 {
                let o1 = self.fetch_data(m1);

                if o1 > 0 {
                    return o1;
                }
            } else if op == 5 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                if o1 != 0 {
                    self.iter = o2 as usize;
                }
            } else if op == 6 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                if o1 == 0 {
                    self.iter = o2 as usize;
                }
            } else if op == 7 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                let pos = self.fetch_op();

                if o1 < o2 {
                    self.buf[pos as usize] = 1;
                } else {
                    self.buf[pos as usize] = 0;
                }
            } else if op == 8 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                let pos = self.fetch_op();

                if o1 == o2 {
                    self.buf[pos as usize] = 1;
                } else {
                    self.buf[pos as usize] = 0;
                }
            } else if op == 99 {
                return -1;
            } else {
                break;
            }
        }

        return 0;
    }
}

fn task1(input: &Vec<i32>) {
    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();

    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });
    
    let mut results: Vec<i32> = Vec::new();

    for perm in permutations {
        // println!("permutation: {:?}", perm);
        let mut count = 0;
        let mut intermediate = 0;
        
        for n in perm {
            let mut numbers = input.clone();
            let mut buffer = Buffer {
                buf: numbers,
                iter: 0,
                first: true,
                amp: n,
            };

            let res = buffer.execute(intermediate);

            if res > -1 {
                intermediate = res;
            }
        }
        
        //println!("result: {}", intermediate);
        results.push(intermediate);
    }

    println!("answer1: {:?}", results.iter().max().unwrap());
    
}

fn task2(input: &Vec<i32>) {
    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();

    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });
    
    let mut results: Vec<i32> = Vec::new();

    for perm in permutations {
        let mut computers: Vec<Buffer> = Vec::new();

        for n in perm {
            let mut numbers = input.clone();
            let mut buffer = Buffer {
                buf: numbers,
                iter: 0,
                first: true,
                amp: n,
            };

            computers.push(buffer);
        }

        let mut iterator = 0;
        let mut intermediate = 0;

        loop {
            let res = computers[iterator].execute(intermediate);

            if res == -1 {
                break;
            }

            intermediate = res;
            iterator += 1;

            if iterator > 4 {
                iterator = 0;
            }
        }


        results.push(intermediate);
    }

    println!("answer2: {:?}", results.iter().max().unwrap());
    
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let numbers: Vec<i32> = contents
        .split(",")
        .map(|i| i.trim())
        .map(|i| i.parse::<i32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect();


    task1(&numbers);
    task2(&numbers);
}
