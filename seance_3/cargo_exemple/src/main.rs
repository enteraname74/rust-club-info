fn main() {
    println!("Operation:");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Missing information from user !");

    let cleaned_line: String = line.replace("\n", "");

    let elements: Vec<&str> = cleaned_line.split(" ").collect();

    if elements.len() < 3 {
        panic!("Missing information");
    }

    let first_number: i32 = match elements[0].parse() {
        Ok(number) => number,
        Err(error) => panic!("Found error in first number ! {}", error),
    };

    let second_number: i32 = match elements[2].parse() {
        Ok(number) => number,
        Err(error) => panic!("Found error in second number ! {}", error),
    };

    let operation: &str = elements[1];

    let res: i32 = match operation {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "/" => first_number / second_number,
        "*" => first_number * second_number,
        _ => panic!("This operation is not valid!"),
    };
    println!("Result: {} = {}", cleaned_line, res);
}
