use rand::Rng;
enum ValidationResult {
    Lower,
    Higher,
    Correct,
}

fn parse_number(number : &str) -> i32 {
    let parsed = number.trim().parse::<i32>()
    .expect("Por favor, escribe un número válido");
    parsed
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
    println!("Escribe 'Q' para salir del juego");
    let number_guess = rand::thread_rng().gen_range(1..=1000);
    loop{
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        if guess.trim() != "Q" {
            match validate_number(parse_number(&guess), number_guess) {
                ValidationResult::Lower => println!("El número es menor"),
                ValidationResult::Higher => println!("El número es mayor"),
                ValidationResult::Correct => {
                    println!("¡Adivinaste!");
                    break;
                }
            }
        }
        else {
            break;
        }
    }
}
