use itertools::Itertools;
use std::fmt;

fn main() {
    let read_file_into_list = utils::read_file_into_list("day7/puzzle1/input");

    let input: Vec<(Equation, Vec<char>)> = read_file_into_list
        .into_iter()
        .map(|x| parse_line(x.as_str()))
        .map(|x| (find_valid_operator(&x), x))
        .filter(|x| x.0.is_some())
        .map(|x|(x.1, x.0.unwrap()))
        .collect();

     let sum:i64 = input.iter()
        .map(|x| x.0.result)
        .sum();

    println!("{:?}", sum);
}

fn find_valid_operator(input: &Equation) -> Option<Vec<char>> {
    let x = vec!['+', '*', '|'];
    for operator in (0..input.equation.len() - 1)
        .map(|_| x.clone().into_iter())
        .multi_cartesian_product()
    {
        if is_valid_operator(input, &operator) {
            return Some(operator);
        }
    }

    return None;
}

fn is_valid_operator(input: &Equation, operator: &Vec<char>) -> bool {
    let mut erg = input.equation[0];

    for i in 1..input.equation.len() {
        if operator[i-1] == '+' {
            erg = erg + input.equation[i]
        }
        else if operator[i-1] == '*' {
            erg = erg * input.equation[i]
        }
        else {
            erg = (erg.to_string() + &input.equation[i].to_string()).parse::<i64>().unwrap();
        }
    }
    return erg == input.result;
}

fn parse_line(line: &str) -> Equation {
    let split: Vec<&str> = line.split(":").collect();
    let result = split[0].parse::<i64>().expect(line);
    let equation: Vec<i64> = split[1]
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    return Equation {
        result: result,
        equation: equation,
    };
}

struct Equation {
    result: i64,
    equation: Vec<i64>,
}

impl fmt::Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        return f
            .debug_struct("Equation")
            .field("result", &self.result)
            .field("equation", &self.equation)
            .finish();
    }
}
