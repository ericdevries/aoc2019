use std::collections::HashMap;
use std::fs;

struct Buffer {
    iter: usize,
    first: bool,
    amp: i64,
    buf: Vec<i64>,
    rel: i64,
    memory: HashMap<i64, i64>,
}

impl Buffer {
    fn fetch_op(&mut self, mode: i64) -> i64 {
        if self.iter >= self.buf.len() {
            return 99;
        }

        let val = self.get_memory(self.iter as i64);
        self.iter += 1;
        return val;
    }

    fn get_address(&mut self, mode: i64) -> i64 {
        let val = self.fetch_op(mode);

        match mode {
            2 => self.rel + val,
            _ => val

            // 1 => val,
            // _ => self.get_memory(val),
        }
    }

    fn get_memory(&mut self, loc: i64) -> i64 {
        let result = match self.memory.get(&(loc as i64)) {
            Some(&x) => x,
            None if loc < (self.buf.len() as i64) => self.buf[loc as usize],
            _ => 0,
        };
        // println!("memory get: {} -> {}", loc, result);
        return result;
    }

    fn set_memory(&mut self, loc: i64, val: i64) {
        //println!("SET memory: {} <- {}", loc, val);
        self.memory.insert(loc, val);
        // self.memory[&loc] = val;
    }

    fn fetch_data(&mut self, mode: i64) -> i64 {
        let op = self.fetch_op(mode);
        let val = match mode {
            0 => self.get_memory(op),
            1 => op,
            2 => self.get_memory(self.rel + op),
            _ => op,
        };

        return val;
    }

    fn execute(&mut self, input_value: i64) -> i64 {
        loop {
            let op_raw = self.fetch_op(0);

            let op = op_raw % 100;
            let m1 = op_raw % 1000 / 100;
            let m2 = op_raw % 10000 / 1000;
            let m3 = op_raw % 100000 / 10000;
            
            // println!("op: {}, {}, {}, {}", op, m1, m2, m3);
            if op == 1 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                let pos = self.get_address(m3);
                self.set_memory(pos, o1 + o2);
            } else if op == 2 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                let pos = self.get_address(m3);
                self.set_memory(pos, o1 * o2);
            } else if op == 3 {
                let o1 = self.get_address(m1);
                self.set_memory(
                    o1,
                    match self.first {
                        true => self.amp,
                        false => input_value,
                    },
                );
                self.first = false;
            } else if op == 4 {
                let o1 = self.fetch_data(m1);
                println!("output: {}", o1);
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
                let pos = self.get_address(m3);

                if o1 < o2 {
                    self.set_memory(pos, 1);
                } else {
                    self.set_memory(pos, 0);
                }
            } else if op == 8 {
                let o1 = self.fetch_data(m1);
                let o2 = self.fetch_data(m2);
                let pos = self.get_address(m3);

                if o1 == o2 {
                    self.set_memory(pos, 1);
                } else {
                    self.set_memory(pos, 0);
                }
            } else if op == 9 {
                let pos = self.fetch_data(m1);
                self.rel += pos;
            } else if op == 99 {
                return -1;
            } else {
                break;
            }
        }

        return 0;
    }
}

fn task1(input: &Vec<i64>) {
    println!("numbers: {:?}", input);
    let mut buffer = Buffer {
        buf: input.clone(),
        iter: 0,
        rel: 0,
        first: false,
        amp: 0,
        memory: HashMap::new(),
    };

    let res = buffer.execute(1);
    println!("answer1: {}", res);
    let res = buffer.execute(2);
    println!("answer2: {}", res);
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let numbers: Vec<i64> = contents
        .split(",")
        .map(|i| i.trim())
        .map(|i| i.parse::<i64>())
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .collect();

    task1(&numbers);
    //task2(&numbers);
}
