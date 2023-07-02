use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();

        print!("Enter an expression(Please space out between the operands and the operator): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let result = calculate(&input);
        println!("Result: {:?}", result);
    }
}

fn calculate(expression: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expression.trim().split_whitespace().collect();
    
   
    let operand1: f64 = tokens[0].parse().unwrap();
    let operator: char = tokens[1].chars().next().unwrap();
    let operand2: f64 = tokens[2].parse().unwrap();
    
    match operator {
        '+' => Ok(operand1 + operand2),
        '-' => Ok(operand1 - operand2),
        '*' => Ok(operand1 * operand2),
        '/' => Ok(operand1 / operand2),
        _ => Err("Invalid Operator !".to_string()),
    }
}



