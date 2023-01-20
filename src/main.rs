use std::io;

struct OperationalVariable { operational_string: String }

impl OperationalVariable {
    fn operation_string_to_number(&self) -> i32 {
        let operation_string = &self.operational_string;
        let string_to_number: i32 = operation_string
            .trim()
            .parse()
            .unwrap();

        string_to_number
    } 

    fn operation_string_to_str(&self) -> &str {
        let operation_string = &self.operational_string;
        operation_string
            .as_str()
            .trim()
    }
}

fn main() -> io::Result<()> {
    let hyphen_sign_decoration = "-".repeat(10); 
    // 27 is equal ESC as a char
    let terminal_key_cleaner = 27 as char;
    let mut matematical_operator = OperationalVariable {operational_string: String::new()};
    let mut first_operation_number = OperationalVariable {operational_string: String::new()};
    let mut second_operation_number = OperationalVariable {operational_string: String::new()};

    print!("{}c", terminal_key_cleaner);

    println!("{}", hyphen_sign_decoration);
    println!("CALCULATOR");
    println!("{}", hyphen_sign_decoration);

    println!("+/-* Which operator you want to use: ");
    io::stdin().read_line(&mut matematical_operator.operational_string)?;

    println!("Tell me the first number: ");
    io::stdin().read_line(&mut first_operation_number.operational_string)?;

    println!("Tell me the seconds number: ");
    io::stdin().read_line(&mut second_operation_number.operational_string)?;

    let operation_result: i32 = match matematical_operator.operation_string_to_str() {
        "-" => first_operation_number.operation_string_to_number() - second_operation_number.operation_string_to_number(),
        "+" => first_operation_number.operation_string_to_number() + second_operation_number.operation_string_to_number(),
        "/" => first_operation_number.operation_string_to_number() / second_operation_number.operation_string_to_number(),
        "*" => first_operation_number.operation_string_to_number() * second_operation_number.operation_string_to_number(),
        _ => 0,
    };

    println!("O resultado é: {}", operation_result);

    Ok(())
}
