//! A simple command-line calculator that performs basic arithmetic operations.
//!
//! # Usage
//!
//! ```
//! cargo run -- --operation <operation> --first <operand1> --second <operand2>
//! ```
//!
//! # Examples
//!
//! ```
//! cargo run -- --operation Add --first 2.5 --second 3.7
//! Result: 6.2
//! 
//! cargo run -- - add --first 2.5 --second 3.7
//! Result: 6.2
//! 
//! cargo run -- -o + -f 2.5 -s 3.7
//! Result: 6.2
//!
//! cargo run -- Add 2.5 3.7
//! Result: 6.2
//! 
//! cargo run -- --operation Sub --first 10 --second 4
//! Result: 6
//!
//! cargo run -- --operation Mul --first 3.14 --second 2
//! Result: 6.28
//!
//! cargo run -- --operation Div --first 10 --second 2
//! Result: 5
//! ```

use clap::{Arg, Command};

/// Performs the requested arithmetic operation on the provided operands.
fn calc(operation: &str, operand1: f64, operand2: f64) -> Result<f64, &'static str> {
    let result = match operation {
        "ADD" | "add" | "+" => operand1 + operand2,
        "SUB" | "sub" | "-" => operand1 - operand2,
        "MUL" | "mul" | "*" => operand1 * operand2,
        "DIV" | "div" | "/" => {
            if operand2 == 0.0 {
                return Err("Error: Division by zero");
            }
            operand1 / operand2
        }
        _ => unreachable!(),
    };
    Ok(result)
}

//When unreachable!() is encountered during runtime, it will cause the program to panic with the message "internal error: entered unreachable code". This is a deliberate panic, indicating a logic error in the program.

fn main() {
    let matches = Command::new("CLI Calculator")
        .version("1.0")
        .author("Ganesh Chandra gc@gmail.com")
        .about("Performs basic arithmetic operations")
        .arg(
            Arg::new("operation")
                .help("The arithmetic operation to perform")
                .value_parser(["Add", "Sub", "Mul", "Div", "+", "-", "*", "/"])
                .ignore_case(true)
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("operand1")
                .help("The first operand")
                .required(false)
                .value_parser(clap::value_parser!(f64))
                .index(2),
        )
        .arg(
            Arg::new("operand2")
                .help("The second operand")
                .required(false)
                .value_parser(clap::value_parser!(f64))
                .index(3),
        )
        .arg(
            Arg::new("operation_flag")
                .short('o')
                .long("operation")
                .help("The arithmetic operation to perform")
                .allow_hyphen_values(true)
                .value_parser(["Add", "Sub", "Mul", "Div", "+", "-", "*", "/"])
                .ignore_case(true)
                .required(false),
        )
        .arg(
            Arg::new("operand1_flag")
                .short('f')
                .long("first")
                .help("The first operand")
                .required(false)
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            Arg::new("operand2_flag")
                .short('s')
                .long("second")
                .help("The second operand")
                .required(false)
                .value_parser(clap::value_parser!(f64)),
        )
        .get_matches();

    // Retrieve values from flags or positional arguments

    let operation = match matches.get_one::<String>("operation").or_else(|| matches.get_one::<String>("operation_flag")) {
        Some(op_value) => op_value.to_ascii_lowercase(),
        None => {
            eprintln!("Error: Operation argument missing");
            return;
        }
    };

    let operand1 = matches
        .get_one::<f64>("operand1")
        .cloned()
        .or_else(|| matches.get_one::<f64>("operand1_flag").cloned())
        .unwrap();

    let operand2 = matches
        .get_one::<f64>("operand2")
        .cloned()
        .or_else(|| matches.get_one::<f64>("operand2_flag").cloned())
        .unwrap();

    match calc(&operation, operand1, operand2) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = calc("add", 2.5, 3.7).unwrap();
        assert_eq!(result, 6.2);
    }

    #[test]
    fn test_addition_plus_sign() {
        let result = calc("+", 2.5, 3.7).unwrap();
        assert_eq!(result, 6.2);
    }

    #[test]
    fn test_addition_addition_uppercase() {
        let result = calc("ADD", 2.5, 3.7).unwrap();
        assert_eq!(result, 6.2);
    }

    #[test]
    fn test_subtraction() {
        let result = calc("sub", 10.0, 4.0).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_subtraction_minus_sign() {
        let result = calc("-", 10.0, 4.0).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_subtraction_uppercase() {
        let result = calc("SUB", 10.0, 4.0).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_multiplication() {
        let result = calc("mul", 3.14, 2.0).unwrap();
        assert_eq!(result, 6.28);
    }

    #[test]
    fn test_division() {
        let result = calc("div", 10.0, 2.0).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let result = calc("div", 10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error: Division by zero");
    }
}
