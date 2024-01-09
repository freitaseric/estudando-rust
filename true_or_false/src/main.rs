use std::io::stdin;

fn main() {
    let mut result: bool = false;

    println!("True Or False");
    println!("Faça comparações e verifique se são verdadeiras ou falsas.");

    println!("Informe o primeiro elemento de comparação:");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("An error has ocurred while trying read next line.");
    let first_element = buffer.replace("\n", "");

    println!("Informe o tipo de comparação: (= | != | > | < | >= | <=)");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("An error has ocurred while trying read next line.");
    let comparation_type = buffer.replace("\n", "");
    if !"=<>!".contains(&comparation_type) {
        println!("Esse tipo de comparação é inválido!");
        return;
    }

    println!("Informe o segundo elemento de comparação:");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("An error has ocurred while trying read next line.");
    let second_element = buffer.replace("\n", "");

    if comparation_type == "=" {
        result = first_element == second_element
    } else if comparation_type == "!=" {
        result = first_element != second_element
    } else if comparation_type == "<" {
        result = first_element.parse::<i32>().unwrap() < second_element.parse::<i32>().unwrap()
    } else if comparation_type == ">" {
        result = first_element.parse::<i32>().unwrap() > second_element.parse::<i32>().unwrap()
    } else if comparation_type == "<=" {
        result = first_element.parse::<i32>().unwrap() <= second_element.parse::<i32>().unwrap()
    } else if comparation_type == ">=" {
        result = first_element.parse::<i32>().unwrap() >= second_element.parse::<i32>().unwrap()
    } else {
        result = first_element == second_element
    }

    println!("{first_element} {comparation_type} {second_element} -> {result}")
}
