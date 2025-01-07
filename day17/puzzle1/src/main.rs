use std::{collections::VecDeque, u64};

fn main() {
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

    let mut stack = VecDeque::new();
    for i in 0..((2 as u64).pow(10)) {
        let mut p = Program {
            a: i,
            b,
            c,
            program: program.clone(),
            ip: 0,
            out: vec![],
        };
        p.run();
        if p.out[0] == p.program[0] {
            println!("{i}, {:?}", p.out);
            stack.push_back((0, i));
        }
    }

    let mut result = (0, 0);
    'outer: while let Some(pop) = stack.pop_front() {
        let (i, a) = pop;
        for j in 0..((2 as u64).pow(10)) {
            let a_new = (j << (3 * (i + 1))) + a % ((2 as u64).pow(3 * (i + 1)));
            
            if stack.contains(&((i+1), a_new)) {
                continue;
            }
            let mut p = Program {
                a: a_new,
                b,
                c,
                program: program.clone(),
                ip: 0,
                out: vec![],
            };
            p.run();
            let mut valid = true;
            let mut valid_till_index = 0;
            for x in 0..p.out.len().min(i as usize + 2) {
                if p.out[x] != p.program[x] {
                    valid = false;
                    break;
                }
                valid_till_index += 1;
            }

            if valid {
                if valid_till_index == p.program.len() && p.out.len() == p.program.len() {
                    println!("{a_new}, {:?}", p.out);
                    result = (i + 1, a_new);
                    break 'outer;
                } else {
                    println!("{i}, {a_new}, {:?}", p.out);
                    stack.push_back((i + 1, a_new));
                }
            }
        }
    }

    println!("{:?}", result);

    let mut p = Program {
        a: 251341328432143,
        b,
        c,
        program: program.clone(),
        ip: 0,
        out: vec![],
    };
    p.run();
    println!("{:?}", p.out);
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
