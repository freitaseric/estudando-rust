use std::collections::HashMap;

pub fn media(numeros: &Vec<i32>) -> i32 {
    let mut soma = 0;
    for numero in numeros {
        soma += numero
    }
    let quantidade: i32 = numeros.len().try_into().unwrap();

    soma / quantidade
}

pub fn mediana(numeros: &Vec<i32>) -> i32 {
    let mut numeros = numeros.clone();
    numeros.sort();

    let mut numero_do_meio = numeros[numeros.len() / 2];

    if numero_do_meio % 2 != 0 {
        numero_do_meio = numero_do_meio.abs()
    }

    numero_do_meio
}

pub fn moda(numeros: &Vec<i32>) -> Vec<(&i32, i32)> {
    let mut mapa = HashMap::new();
    for numero in numeros {
        let contagem = mapa.entry(numero).or_insert(0);
        *contagem += 1;
    }

    let contagem_maxima = mapa.values().cloned().max().unwrap_or(0);
    mapa.into_iter()
        .filter(|&(_, valor)| valor == contagem_maxima)
        .map(|(chave, valor)| (chave, valor))
        .collect()
}
