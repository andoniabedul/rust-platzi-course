fn first_approach() {
    let mut name_input: String = String::new();
    let mut surname_input: String = String::new();
    let mut age_input: String = String::new(); 
    println!("Por favor, introduce tu nombre.");
    std::io::stdin().read_line(&mut name_input).unwrap();
    println!("Ahora, introduce tu apellido.");
    std::io::stdin().read_line(&mut surname_input).unwrap();
    println!("Genial, ahora dime tu edad");
    std::io::stdin().read_line(&mut age_input).unwrap();
    
    let name: String = name_input.trim().to_string();
    let surname: String = surname_input.trim().to_string();
    let age: u8 = age_input.trim().parse().unwrap();
    
    println!("Hola, {} {}, bievenido. Tu edad es {}", name, surname, age);
}

fn main(){
    let mut name_input: String = String::new();
    let mut city_input: String = String::new();

    println!("Hola, dime tu nombre!");
    std::io::stdin().read_line(&mut name_input).unwrap();
    println!("Genial, ¿de que ciudad vienes?");
    std::io::stdin().read_line(&mut city_input).unwrap();

    let name: String = name_input.trim().to_string();
    let city: String = city_input.trim().to_string();

    println!("Un gusto en conocerte, {}. ¿Como es la vida en {}?", name, city);

}
