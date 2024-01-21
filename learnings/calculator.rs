use regex::Regex;

fn main(){

    println!("Pruebita");

    // Regex Expressions
    // Sum: (\d+)\s?\+\s?(\d+)
    // Multiply: (\d+)\s?\*\s?(\d+)
    // Subtraction: (\d+)\s?\-\s?(\d+)
    // Division: (\d+)\s?\/\s?(\d+)
    

    let regex_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_multiply = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let regex_subtraction = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let regex_division = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    // User data
    println!("Introduce tu expresion");
    let mut expression = String::new();

    std::io::stdin().read_line(&mut expression).unwrap();

    loop {
        let mut operation = String::new();
        let mut regex = Regex::new();
        let is_addition: bool = !regex_add.captures(expression.as_str()).is_none();
        let is_multiply: bool = !regex_multiply.captures(expression.as_str()).is_none();
        let is_subtraction: bool = !regex_subtraction.captures(expression.as_str()).is_none();
        let is_division: bool = !regex_division.captures(expression.as_str()).is_none();

        // Operations
        if is_addition {
            operation = "SUM".to_string();
            regex = regex_add;
        } else if is_multiply {
            operation = "MULT".to_string();
            regex = regex_multiply;
        } else if is_subtraction {
            operation = "SUBS".to_string();
            regex = regex_subtraction;
        } else if is_division  {
            operation = "DIV".to_string();
            regex = regex_division;
        } else {
            break;           
        }
        println!("Operation: {}", operation.as_str());

        expression =  make_operation(regex, expression, operation.clone());      
        // Result
        println!("El resultado es {}", expression);

    }
}

fn make_operation(regex: Regex, expression: String, type_of_operation: String) -> String {
    let regex_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_multiply = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let regex_subtraction = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let regex_division = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    let operation: i32;
    

    if type_of_operation == "MULT" {
        let caps = regex_multiply.captures(expression.as_str());
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        operation = left_value * right_value;
        return expression.replace(cap_expression, &operation.to_string());

    } else if type_of_operation == "DIV" {
        let caps = regex_division.captures(expression.as_str());
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        operation = left_value / right_value;
        return expression.replace(cap_expression, &operation.to_string());

    } else if type_of_operation == "SUM" {
        let caps = regex_add.captures(expression.as_str());
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        operation = left_value + right_value;
        return expression.replace(cap_expression, &operation.to_string());

    } else if type_of_operation == "SUBS" {
        let caps = regex_subtraction.captures(expression.as_str());
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        operation = left_value - right_value;
        return expression.replace(cap_expression, &operation.to_string());

    }  else {
        println!("No operation found");
        return "".to_string();
    }
}
