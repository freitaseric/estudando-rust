use crate::{algebra::{media, mediana, moda}, empresa::Empresa};

mod algebra;
mod empresa;

fn main() {
    println!("Testes do módulo de álgebra:\n");
    teste_algebra();
    println!("\n===== FIM DO TESTE =====\n\n\n");
    
    println!("Testes do módulo de gerenciamento de empresa:\n");
    teste_empresa();
    
    println!("\n===== FIM DO TESTE =====");
}

fn teste_algebra() {
    let vetor = vec![1, 2, 54, 10, 10, 10, 54, 12, 45, 42, 345, 4345];

    println!("A média de {:?} é {:?}", vetor, media(&vetor));
    println!("A mediana é {}", mediana(&vetor));
    println!("A(s) moda(s) dessa lista de números é(são): {:?}", moda(&vetor));
}

fn teste_empresa() {
    let mut empresa = Empresa::new();

    empresa.adicionar_funcionario(String::from("Vinícius"), String::from("TI"));
    empresa.adicionar_funcionario(String::from("Aline"), String::from("Arquitetura"));
    empresa.adicionar_funcionario(String::from("Isabelle"), String::from("Engenharia"));
    empresa.adicionar_funcionario(String::from("Eric"), String::from("TI"));

    let funcionarios_ti = empresa.listar_funcionarios_por_departamento(&String::from("TI"));
    let funcionarios_engenharia =
        empresa.listar_funcionarios_por_departamento(&String::from("Engenharia"));
    let funcionarios_arquitetura =
        empresa.listar_funcionarios_por_departamento(&String::from("Arquitetura"));

    println!("Os funcionários de TI são: {:?}", funcionarios_ti);
    println!(
        "Os funcionários de Engenharia são: {:?}",
        funcionarios_engenharia
    );
    println!(
        "Os funcionários de Arquitetura são: {:?}",
        funcionarios_arquitetura
    );

    println!("{:?}", empresa.listar_funcionarios())
}