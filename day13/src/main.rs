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
    fn fetch_op(&mut self) -> i64 {
        if self.iter >= self.buf.len() {
            return 99;
        }

        let val = self.get_memory(self.iter as i64);
        self.iter += 1;
        return val;
    }

    fn get_address(&mut self, mode: i64) -> i64 {
        let val = self.fetch_op();

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
        let op = self.fetch_op();
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
            let op_raw = self.fetch_op();

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
                return o1;
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
                break;
            } else {
                break;
            }
        }

        return -1;
    }
}

fn task1(input: &Vec<i64>) {
    let mut buffer = Buffer {
        buf: input.clone(),
        iter: 0,
        rel: 0,
        first: false,
        amp: 0,
        memory: HashMap::new(),
    };

    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    loop {
        let x = buffer.execute(0);
        let y = buffer.execute(0);
        let t = buffer.execute(0);

        if x < 0 {
            break;
        }

        map.insert((x, y), t);
    }

    let count = map
        .values()
        .filter(|&x| *x == (2 as i64))
        .collect::<Vec<&i64>>()
        .len();

    draw(&map);
}

fn task2(input: &Vec<i64>) {
    let mut input = input.clone();
    input[0] = 2;

    let mut buffer = Buffer {
        buf: input.clone(),
        iter: 0,
        rel: 0,
        first: false,
        amp: 0,
        memory: HashMap::new(),
    };

    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut paddle = 21;
    let mut joystick = 0;

    loop {
        let x = buffer.execute(joystick);
        let y = buffer.execute(joystick);
        let t = buffer.execute(joystick);
        //println!("order: {} x {} -> {}", x, y, t); 
        if x < 0 && y < 0 {
            break;
        } else  if x < 0 && y == 0{
            println!("score: {}", t);
        } else {
            map.insert((x, y), t);

            if t == 3 {
                // println!("paddle is at {} x {}", x, y);
                paddle = x;
            }

            if t == 4 {
                // println!("ball is at {} x {}", x, y);

                if x < paddle {
                    joystick = -1;
                } else if x > paddle {
                    joystick = 1;
                } else {
                    joystick = 0;
                }
            }
        }
    }

    let count = map
        .values()
        .filter(|&x| *x == (2 as i64))
        .collect::<Vec<&i64>>()
        .len();

    draw(&map);
}


fn draw(chart: &HashMap<(i64, i64), i64>) {
    let xs: Vec<i64> = chart.keys().map(|x| x.0).collect();
    let ys: Vec<i64> = chart.keys().map(|x| x.1).collect();

    let minx = xs.iter().min().unwrap();
    let maxx = xs.iter().max().unwrap();
    let miny = ys.iter().min().unwrap();
    let maxy = ys.iter().max().unwrap();

    for y in *miny..=*maxy {
        for x in *minx..=*maxx {
            let c = match chart.get(&(x, y)) {
                Some(&x) => x,
                None => 0,
            };

            print!("{}", c);
        }

        println!();
    }
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

    //task1(&numbers);
    task2(&numbers);
}
