use std::env::{args, Args};

fn main() {

    let mut args: Args = args();
    // nth takes index returns Option (unwrap it), also removes element accessed

    // The first argument is the location of the compiled binary, so skip it
    let first: String = args.nth(1).unwrap();
    // After accessing the second argument, the iterator's next element becomes the first (AHH)
    let operator: String = args.nth(0).unwrap();
    let second: String = args.nth(0).unwrap();

    println!("Calculating {} {} {}", first, operator, second);
    let first_number: f32 = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse::<f32>().unwrap();
    let op : char = operator.chars().next().unwrap();

    let result : f32 = operate(op, first_number, second_number);

    println!("{}", format!("{} {} {} = {}", first_number, op, second_number, result));
}

fn operate(operator: char, first: f32, second: f32) -> f32 {
    match operator {
        '+' => first + second,
        '-' => first + second,
        '*' => first * second,
        '/' => first / second,
        _ => panic!("Invalid Operator Used"),
    }
}
