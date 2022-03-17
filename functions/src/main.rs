use colored::Colorize;

fn main() {
    println!("🔢 Functions with numbers!");

    let sum_result = sum(2, 2);
    println!(
        "The result of the sum is {}",
        sum_result.to_string().green()
    );

    let sub_result = sub(4, 2);
    println!(
        "The result of the subtraction is {}",
        sub_result.to_string().green()
    );

    let multiply_result = multiply(4, 2);
    println!(
        "The result of the multiplication is {}",
        multiply_result.to_string().green()
    );

    // We don't generate a new variable here since we're handling
    // the result within the 'match' scope
    match divide(10, 4) {
        Ok(result) => println!(
            "The result of the division is {}",
            result.to_string().green()
        ),
        Err(msg) => println!("{}", msg),
    };

    // We don't generate a new variable here since we're handling
    // the result within the 'match' scope
    match divide(10, 0) {
        Ok(result) => println!(
            "The result of the division is {}",
            result.to_string().green()
        ),
        Err(msg) => println!("{}", msg),
    };
}

fn sum(a: i32, b: i32) -> i32 {
    println!(
        "Sum {} and {}",
        a.to_string().green(),
        b.to_string().green()
    );
    // Rust functions has implicit return on the last line,
    // this way, we can ommit: return x + y
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    println!(
        "Subtract {} from {}",
        b.to_string().green(),
        a.to_string().green()
    );
    // Rust functions has implicit return on the last line,
    // this way, we can ommit: return x + y
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    println!(
        "Multiply {} times {}",
        a.to_string().green(),
        b.to_string().green()
    );
    // Rust functions has implicit return on the last line,
    // this way, we can ommit: return x + y
    a * b
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    println!(
        "Dive {} by {}",
        a.to_string().green(),
        b.to_string().green()
    );
    if b == 0 {
        {
            Err("Cannot divide a number by zero (0)")
        }
    } else {
        // Rust functions has implicit return on the last line,
        // this way, we can ommit: return x + y
        {
            Ok(a / b)
        }
    }
}
