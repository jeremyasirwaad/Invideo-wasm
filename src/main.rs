use meval;

fn calculate(expression: String) -> String {
    let result = match meval::eval_str(&expression) {
        Ok(val) => val.to_string(),
        Err(_) => "Error".to_string(),
    };
    result
}

fn main() {
    let expression = String::from("(2 + 3) * 4");
    let result = calculate(expression);
    println!("Result: {}", result);
}
