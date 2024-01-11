extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Bem vindo ao Jogo!");
    println!("Adivinhe em qual número estou pensando...");

    loop {
        println!("Dê seu palpite: (um número positivo entre 1 e 100)");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler entrada");

        println!("Seu palpite foi: {}", guess);

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor digite um número!\n\n\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
            Ordering::Greater => println!("Muito alto!"),
        }
    }
}
