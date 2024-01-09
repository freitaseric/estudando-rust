use std::io::stdin;

const QUESTION_1: &str = "Seu personagem é homem? (s/n/t)";
const QUESTION_2: &str = "Seu personagem mora no brasil? (s/n/t)";
const QUESTION_3: &str = "Seu personagem mora com você? (s/n/t)";

fn main() {
    let mut buffer = String::new();

    print("Bem vindo(a) ao Akinator!");
    print("Deixe-me ler sua mente...");
    print("Pense em um personagem: (escreva-o aqui)");

    stdin()
        .read_line(&mut buffer)
        .expect("An error ocurred while trying read line");

    let character = buffer.replace("\n", "");
    question1();
    print("Já sei!!!");
    println!("Seu personagem é: {character}")
}

fn print(args: &str) {
    println!("{args}")
}

fn question1() {
    let mut buffer = String::new();

    print("Certo! Vamos começar");
    print(QUESTION_1);

    stdin()
        .read_line(&mut buffer)
        .expect("An error ocurred while trying read line");

    question2()
}
fn question2() {
    let mut buffer = String::new();

    print("Hummm, acho que estou chegando perto!");
    print(QUESTION_2);

    stdin()
        .read_line(&mut buffer)
        .expect("An error ocurred while trying read line");

    question3()
}
fn question3() {
    let mut buffer = String::new();

    print("Vamos para última pergunta, estou sentindo que já sei quem é...");
    print(QUESTION_3);

    stdin()
        .read_line(&mut buffer)
        .expect("An error ocurred while trying read line");
}
