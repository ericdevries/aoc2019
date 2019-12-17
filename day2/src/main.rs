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

    fn fetch_data(&mut self) -> i32 {
        let op = self.fetch_op();
        return self.buf[op as usize];
    }

    fn execute(&mut self) {
        loop {
            let op = self.fetch_op();

            if op == 1 {
                let o1 = self.fetch_data();
                let o2 = self.fetch_data();
                let pos = self.fetch_op();
                self.buf[pos as usize] = o1 + o2;
            } else if op == 2 {
                let o1 = self.fetch_data();
                let o2 = self.fetch_data();
                let pos = self.fetch_op();
                self.buf[pos as usize] = o1 * o2;
            } else if op == 99 {
                break;
            } else {
                break;
            }
        }
    }

    fn print_state(&self) {
        println!("state: {:?}", self.buf);
    }
}

fn day2() {
    let data = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,2,9,19,23,1,23,6,27,1,13,27,31,1,31,10,35,1,9,35,39,1,39,9,43,2,6,43,47,1,47,5,51,2,10,51,55,1,6,55,59,2,13,59,63,2,13,63,67,1,6,67,71,1,71,5,75,2,75,6,79,1,5,79,83,1,83,6,87,2,10,87,91,1,9,91,95,1,6,95,99,1,99,6,103,2,103,9,107,2,107,10,111,1,5,111,115,1,115,6,119,2,6,119,123,1,10,123,127,1,127,5,131,1,131,2,135,1,135,5,0,99,2,0,14,0";

    let numbers: Vec<i32> = data
        .split(",")
        .map(|i| i.parse::<i32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut copied = numbers.clone();
            copied[1] = noun;
            copied[2] = verb;

            let mut buffer = Buffer {
                buf: copied,
                iter: 0,
            };

            buffer.execute();

            if buffer.buf[0] == 19690720 {
                println!("noun {:?}, verb {:?}", noun, verb);
                println!("answer is: {:?}", 100 * noun + verb);
            }
        }
    }
}

fn day1() {
    let data = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,2,9,19,23,1,23,6,27,1,13,27,31,1,31,10,35,1,9,35,39,1,39,9,43,2,6,43,47,1,47,5,51,2,10,51,55,1,6,55,59,2,13,59,63,2,13,63,67,1,6,67,71,1,71,5,75,2,75,6,79,1,5,79,83,1,83,6,87,2,10,87,91,1,9,91,95,1,6,95,99,1,99,6,103,2,103,9,107,2,107,10,111,1,5,111,115,1,115,6,119,2,6,119,123,1,10,123,127,1,127,5,131,1,131,2,135,1,135,5,0,99,2,0,14,0";

    let numbers: Vec<i32> = data
        .split(",")
        .map(|i| i.parse::<i32>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect();

    let mut buffer = Buffer {
        buf: numbers,
        iter: 0,
    };

    buffer.execute();
    buffer.print_state();
}

fn main() {
    day1();
    day2();
    println!("Hello, world!");
}
