fn main(){
    
    let mut names: Vec<String> = Vec::new();

    for i in 0..3 {
        
        println!("Por favor introduce un nombre: ");

        let mut name_input: String = String::new();
        std::io::stdin().read_line(&mut name_input).unwrap();
        
        let name: String = name_input.trim().to_string();
        
        names.push(name);
    }

    println!("Hay {} nombres", names.len());
    for name in names {
        println!("Nombre: {:?}", name);
    }


}


