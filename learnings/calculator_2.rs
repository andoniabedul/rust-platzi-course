use regex::Regex;

const OPERATIONS: [&str; 4] = ["*", "/", "+", "-"];

fn main(){

    println!("Calculadora");
    let regex_general: Regex = Regex::new(r"(\d+)\s?(\+|\-|\*|\/)\s?(\d+)").unwrap();
    
    let mut expression: String = get_operation_from_user();
    
    loop {
        expression = make_operation(regex_general.clone(), expression);
    }
    println!("El resultado final es {}", expression);
}


fn get_operation_from_user() -> String {
    println!("Introduce tu expresion");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    return expression;
}


fn make_operation(regex: Regex, expression: String) -> String {
    let operation: i32;
    let caps = regex.captures(expression.as_str());
    let caps = caps.unwrap();

    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
    let type_of_operation: String = caps.get(2).unwrap().as_str().parse().unwrap();
    
    if type_of_operation == OPERATIONS[0] {
        operation = left_value * right_value;
        return expression.replace(cap_expression, &operation.to_string());
    } else if type_of_operation == OPERATIONS[1] {
        operation = left_value / right_value;
        return expression.replace(cap_expression, &operation.to_string());
    } else if type_of_operation == OPERATIONS[2] {
        operation = left_value + right_value;
        return expression.replace(cap_expression, &operation.to_string());
    } else if type_of_operation == OPERATIONS[3] {
        operation = left_value - right_value;
        return expression.replace(cap_expression, &operation.to_string());
    }  else {
        println!("No operation found");
        return "".to_string();
    }
}

