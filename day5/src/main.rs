use std::fs;

struct Buffer {
    iter: usize,
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
                self.buf[o1 as usize] = input_value;
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
                break;
            } else {
                break;
            }
        }

        return 0;
    }

    fn print_state(&self) {
        println!("state: {:?}", self.buf);
    }
}

fn task1(numbers: &Vec<i32>) {
    let mut buffer = Buffer {
        buf: numbers.clone(),
        iter: 0,
    };

    let output = buffer.execute(1);
    println!("answer1: {}", output);
    // buffer.print_state();
}

fn task2(numbers: &Vec<i32>) {
    let mut buffer = Buffer {
        buf: numbers.clone(),
        iter: 0,
    };

    let output = buffer.execute(5);
    println!("answer1: {}", output);
    // buffer.print_state();
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

    println!("input: {:?}, size is {}", numbers, numbers.len());
    task1(&numbers);
    task2(&numbers);
}
