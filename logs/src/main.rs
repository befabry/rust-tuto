use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    basic_error_handling();
    nested_match_statements();
    expects();

    // ? is the Try operator
    // Idiomatic Rust - concise but still propagates errors properly
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())
}

// don't handle panic
fn expects() -> () {
    let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors_expect.txt", error_logs.join("\n"))
        .expect("failed to write errors_expect.txt");
}

// handles panic but nested / hard to read
fn nested_match_statements() -> () {
    let mut error_logs: Vec<String> = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_logs = extract_errors(text.as_str());

            match fs::write("errors_nested.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors_nested.txt"),
                Err(error) => println!("Writing of errors_nested.txt failed: {}", error),
            }
        }
        Err(err) => println!("Failed to read file : {}", err),
    }
    println!("{:#?}", error_logs);
}

// Returns Vec<String> - a collection of owned strings that live on the heap
// This allows the error lines to outlive the original text and be passed around freely
fn extract_errors(text: &str) -> Vec<String> {
    let splt_text = text.split("\n");

    let mut results: Vec<String> = vec![];
    for line in splt_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn basic_error_handling() -> () {
    // Pattern 1: Match with both value and error
    // Use when you need to access the successful value
    match divide(5.0, 0.0) {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err),
    }

    // Pattern 2: Match with void value (using ..)
    // Use when you only care about success/failure, not the actual value
    // The .. pattern ignores the content when we don't need it
    match validate_email(String::from("asdf@asdf.com")) {
        Ok(..) => println!("email is valid"),
        Err(err) => println!("{}", err),
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(()) // returning an empty Tuple
    } else {
        Err(Error::other("email must have a @"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
