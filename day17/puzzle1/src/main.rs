use std::u64;

fn main() {
    let from = 0b100000000000000000000000000000000000000000000000;
    let to = 0b111111111111111111111111111111111111111111111111;
    let num_of_threads = 128;
    let per_thread = (to - from) / num_of_threads;
    let mut threads = Vec::new();
    for i in 0..num_of_threads {
        let local_from = from + i * per_thread;
        let local_to = from + i * per_thread + per_thread;
        threads.push(std::thread::spawn(move || {
            let input = utils::read_file_into_list("day17/puzzle1/input");

            let a = input[0].split_whitespace().collect::<Vec<&str>>()[2]
                .parse::<u64>()
                .unwrap();
            let b = input[1].split_whitespace().collect::<Vec<&str>>()[2]
                .parse::<u64>()
                .unwrap();
            let c = input[2].split_whitespace().collect::<Vec<&str>>()[2]
                .parse::<u64>()
                .unwrap();
        
            let program = input[4].split_whitespace().collect::<Vec<&str>>()[1]
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            
            let mut p = Program {
                a,
                b,
                c,
                program,
                ip: 0,
                out: vec![],
            };
            for j in local_from..local_to + 1 {
                p.a = j;
                p.run();
                if p.out.len() == p.program.len()
                    && p.out.iter().enumerate().all(|x| *x.1 == p.program[x.0])
                {
                    println!("{}", j);
                }
                p.b = b;
                p.c = c;
                p.out.clear();
                p.ip = 0;
            }
        }));
    }

    for t in threads {
        t.join();
    }
}

#[derive(Debug)]
struct Program {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    ip: usize,
    out: Vec<u64>,
}

static ADV: u64 = 0;
static BXL: u64 = 1;
static BST: u64 = 2;
static JNZ: u64 = 3;
static BXC: u64 = 4;
static OUT: u64 = 5;
static BDV: u64 = 6;
static CDV: u64 = 7;

impl Program {
    fn run(&mut self) {
        while self.ip < self.program.len() - 1 {
            let op = self.program[self.ip];
            let arg = self.program[self.ip + 1];
            let value = self.get_value(arg);
            if op == ADV {
                self.a /= (2 as u64).pow(value as u32);
            } else if op == BXL {
                self.b = self.b ^ arg;
            } else if op == BST {
                self.b = value % 8;
            } else if op == JNZ {
                if self.a != 0 {
                    self.ip = arg as usize;
                } else {
                    self.ip += 2;
                }
            } else if op == BXC {
                self.b = self.b ^ self.c;
            } else if op == OUT {
                self.out.push(value % 8);
            } else if op == BDV {
                self.b = self.a / (2 as u64).pow(value as u32);
            } else if op == CDV {
                self.c = self.a / (2 as u64).pow(value as u32);
            }
            if op != JNZ {
                self.ip += 2;
            }
        }
    }

    fn get_value(&self, value: u64) -> u64 {
        if value <= 3 {
            return value;
        } else if value == 4 {
            return self.a;
        } else if value == 5 {
            return self.b;
        } else if value == 6 {
            return self.c;
        }
        return 0;
    }
}
