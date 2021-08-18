fn main() {
    println!("Enter equation (and pretend that PEDMAS doesn't exist ):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let op = break_down_slice(&input[..]);
    let out = op.calculate();
    println!("Answer: {}", out);
}
//SLICE: a..b
//a inclusive, b exclusive

fn break_down_slice(slice: &str) -> Operation {
    let mut num_index: i32 = -1;
    for i in 0 as u32..slice.len() as u32 {
        let char = slice.chars().nth(i as usize).unwrap();
        if char == '(' {
            let mut parentheses: i16 = 0;
            for j in (i+1)..slice.len() as u32 {
                if char == '(' {
                    parentheses += 1;
                } else if char == ')' {
                    parentheses -= 1;
                    if parentheses == -1 {
                        return Operation::PARENTHESES(Box::from(break_down_slice(&slice[((i + 1) as usize).. j as usize])));

                    }
                }
            }
        }
        
        if char.is_ascii_digit() {
            if num_index == -1 {
                num_index = i as i32;
            }
        } else if char == '^' {
            assert_ne!(num_index, -1);
            return Operation::EXPONENT(Box::from(parse_number(&slice[(num_index as usize)..(i as usize)])), Box::from(break_down_slice(&slice[((i+1) as usize)..(slice.len() as usize)])));
        } else if char == '/' {
            assert_ne!(num_index, -1);
            return Operation::DIVISION(Box::from(parse_number(&slice[(num_index as usize)..(i as usize)])), Box::from(break_down_slice(&slice[((i+1) as usize)..(slice.len() as usize)])));
        } else if char == '*' {
            assert_ne!(num_index, -1);
            return Operation::MULTIPLICATION(Box::from(parse_number(&slice[(num_index as usize)..(i as usize)])), Box::from(break_down_slice(&slice[((i+1) as usize)..(slice.len() as usize)])));
        } else if char == '+' {
            assert_ne!(num_index, -1);
            return Operation::ADDITION(Box::from(parse_number(&slice[(num_index as usize)..(i as usize)])), Box::from(break_down_slice(&slice[((i+1) as usize)..(slice.len() as usize)])));
        } else if char == '-' {
            assert_ne!(num_index, -1);
            return Operation::SUBTRACTION(Box::from(parse_number(&slice[(num_index as usize)..(i as usize)])), Box::from(break_down_slice(&slice[((i+1) as usize)..(slice.len() as usize)])));
        }
    }
    return parse_number(slice);
}


fn parse_number(slice: &str) -> Operation {
    assert!(slice.chars().nth(0).unwrap().is_ascii_digit());
    let mut value: f64 = 0.0;
    let mut decimal_place: f64 = 0.0;
    let mut decimal = false;
    for j in 0..slice.len() {
        let char = slice.chars().nth(j).unwrap();
        if char.is_ascii_digit() {
            if !decimal {
                value *= 10.0_f64;
                value += ((char as i32) - ('0' as i32)) as f64;
            } else {
                value += (((char as i32) - ('0' as i32)) as f64) / decimal_place;
                decimal_place *= 10.0_f64;
            }
        } else if char == '.' {
            assert!(!decimal);
            decimal = true;
            decimal_place = 10.0;
        }
    }
    return Operation::VALUE(value);
}

enum Operation {
    PARENTHESES(Box<Operation>),
    EXPONENT(Box<Operation>, Box<Operation>),
    DIVISION(Box<Operation>, Box<Operation>),
    MULTIPLICATION(Box<Operation>, Box<Operation>),
    ADDITION(Box<Operation>, Box<Operation>),
    SUBTRACTION(Box<Operation>, Box<Operation>),
    VALUE(f64)
}

impl Operation {
    fn calculate(&self) -> f64 {
        return match self {
            Operation::PARENTHESES(o) => {
                o.calculate()
            }
            Operation::EXPONENT(a, b) => {
                a.calculate().powf(b.calculate())
            }
            Operation::DIVISION(a, b) => {
                a.calculate() / b.calculate()
            }
            Operation::MULTIPLICATION(a, b) => {
                a.calculate() * b.calculate()
            }
            Operation::ADDITION(a, b) => {
                a.calculate() + b.calculate()
            }
            Operation::SUBTRACTION(a, b) => {
                a.calculate() - b.calculate()
            }
            Operation::VALUE(a) => {
                *a
            }
        }
    }
}