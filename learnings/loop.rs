fn main(){
    let first_number = 123;
    let second_number = 321;

    let sum = first_number + second_number;
    

    loop {

        println!("Primer número {}. Segundo número {}. Diga la suma de esos dos números", first_number, second_number);

        let mut answer_input: String = String::new();

        std::io::stdin().read_line(&mut answer_input).unwrap();

        let answer: i32= answer_input.trim().parse().unwrap();

        println!("El número de tu respuesta es: {}", answer);

        if answer == sum {
            println!("Respuesta correcta");
            return;
        } else {
            println!("Respuesta incorrecta... Intentalo de nuevo");
        }
    }

}
