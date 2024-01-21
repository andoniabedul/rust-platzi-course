use csv::{ ReaderBuilder, StringRecord };
use std::{ fs };
use std::collections::{ HashMap };


const FILENAME: &str = "resources/history.csv";
const FIRST_ACTION_NAME: &str = "INICIO";

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(filename) {
        Ok(content) => Ok(content),
        Err(error) => return Err(error)
    }
}

#[derive(Debug)]
struct HistoryLine {
    line_type: String,
    action: String,
    line: String,
    life: i32,
    options: Vec<HistoryLine>
}

impl HistoryLine {
    pub fn new(record_line: StringRecord) -> Self {
        let line_type: &str = record_line.get(0).expect("Debe haber un tipo de argumento").trim();
        let action: &str = record_line.get(1).expect("Debe haber un nombre").trim();
        let line: &str = record_line.get(2).expect("Debe haber una linea para mostrar al usuario");
        let life: i32 = record_line.get(3).expect("No se encontró valor para la columna").trim().parse().unwrap_or(0);
        HistoryLine {
            line_type: line_type.to_string(),
            action: action.to_string(),
            line: line.to_string(),
            life: life,
            options: vec![]
        }
    }
}



fn main() {
    let content = read_file(FILENAME).expect("El archivo de historia no fue encontrado"); 
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
    let mut history_lines: HashMap<String, HistoryLine> = HashMap::new();
    let mut life = 100;
    let mut actual_action: &str = FIRST_ACTION_NAME;
    let mut last_action: String = "".to_string();

    for record in reader.records() {
        let record_line = record.expect("La linea tiene algun error");
       
        let history_line = HistoryLine::new(record_line);
        let action: String = history_line.action.clone();

        if history_line.line_type == "SITUACION" {
            history_lines.insert(action.clone(), history_line);
            last_action = action;
        } else if history_line.line_type == "OPCION" {
            if let Some(data) = history_lines.get_mut(&last_action) {
                (*data).options.push(history_line);
            }
        }
    }

    println!("Bienvenido a esta historia. Comienza.");
    
    loop {
        if let Some(line) = history_lines.get(actual_action) {
            life += line.life;
            println!("{}", line.line);
            for (index, option) in line.options.iter().enumerate() {
                println!("[{}] {}", index, option.line);
            }
            let mut input_selection = String::new();
            std::io::stdin().read_line(&mut input_selection).unwrap();
            let selection = input_selection.trim().parse().unwrap_or(99);
            if let Some(selection) = &line.options.get(selection) {
                println!("Has elegido: {} \n", selection.line);
                actual_action = &selection.action;
            } else {
                println!("Esta opción no existe \n");
            }
        } else {
            println!("Ha terminado el juego");
            break;
        }

        if life <= 0 {
            println!("Has perdido... Game over");
            break;
        }
    }
}
