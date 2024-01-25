use rand::Rng;
enum ValidationResult {
    Lower,
    Higher,
    Correct,
}

fn parse_number(number: &str) -> Result<i32, std::num::ParseIntError> {
    number.trim().parse::<i32>()
}


fn validate_number(number_input: i32, number_guess: i32) -> ValidationResult {
    if number_input > number_guess {
        ValidationResult::Lower
    } else if number_input < number_guess {
        ValidationResult::Higher
    } else {
        ValidationResult::Correct
    }
}

fn main() {
    println!("Adivina el número!");
    println!("Los números válidos son del 1 al 1000");
    println!("Escribe 'exit' para salir del juego");
    let number_guess = rand::thread_rng().gen_range(1..=1000);
    loop{
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        if guess.trim().to_lowercase() == "quit" {

            match parse_number(&guess) {
                Ok(number) => {

                    match validate_number(number, number_guess) {
                        ValidationResult::Lower => println!("El número es menor"),
                        ValidationResult::Higher => println!("El número es mayor"),
                        ValidationResult::Correct => {
                            println!("¡Adivinaste!");
                            break;
                        }
                    }

                },
                Err(err) => {
                    println!("Error al analizar el número: {}", err);
                }
            }


        }
        else {
            break;
        }
    }
}
