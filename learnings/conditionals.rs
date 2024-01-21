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

fn disco_enter(){
    let mut name_input: String = String::new();
    let mut city_input: String = String::new();

    println!("Hola, dime tu nombre!");
    std::io::stdin().read_line(&mut name_input).unwrap();
    println!("Genial, ¿de que ciudad vienes?");
    std::io::stdin().read_line(&mut city_input).unwrap();

    let name: String = name_input.trim().to_string();
    let city: String = city_input.trim().to_string();

    println!("Un gusto en conocerte, {}. ¿Como es la vida en {}?", name, city);
    
    let max_age: u8 = 23;
    let mut age_input: String = String::new();

    println!("Espero que bien... Ahora dime tu edad, esta discoteca es solo para mayores de {} años", max_age);
    
    std::io::stdin().read_line(&mut age_input).unwrap();

    let age: u8 = age_input.trim().parse().unwrap();

    if age >= max_age {
        println!("Genial {}, puedes entrar a la discoteca. Adelante.", name);
    } else {
        println!("Lo siento {}, no puedes entrar a la discoteca.", name);
    }

}

fn main(){
    println!("Despierta Neo...");
    println!("...");
    println!("...");
    
    let red_pill: String = String::from("roja");
    let blue_pill: String = String::from("azul");
    let correct_answer: String = String::from(red_pill.clone());
    let mut neo_answer_input: String = String::new();

    
    println!("Neo, tienes dos opciones puedes tomar la pildora {}, que te sacará de este mundo y te mostrará la verdadera realidad", red_pill);
    println!("O tienes la posibilidad de tomar la pildora {}, que dejará todo como está y podrás seguir viviendo tu aventura", blue_pill);
    println!("Escribe {} o {} con tu decisión", red_pill, blue_pill);

    std::io::stdin().read_line(&mut neo_answer_input).unwrap();
    
    let neo_answer: String = neo_answer_input.trim().to_string();

    println!("Has elegido {}", neo_answer);

    if neo_answer == red_pill {
        println!("Excelente decisión Neo, ven conmigo");
    } else if neo_answer == blue_pill {
        println!("Has elegido vivir la mentira. No nos veremos nunca más");
    } else {
        println!("Respuesta incorrecta, la respuesta debe ser {} o {}", red_pill, blue_pill);
    }

    println!("La respuesta correcta siempre fue {}", correct_answer);
}
