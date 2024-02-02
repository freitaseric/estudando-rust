use std::{collections::HashMap, vec};

use itertools::Itertools;

pub struct Empresa {
    funcionarios: HashMap<String, String>,
}

impl Empresa {
    pub fn new() -> Self {
        Empresa {
            funcionarios: HashMap::new(),
        }
    }
}

impl Empresa {
    pub fn adicionar_funcionario(&mut self, nome: String, departamento: String) -> String {
        self.funcionarios
            .entry(nome)
            .or_insert(departamento)
            .to_string()
    }
    pub fn listar_funcionarios_por_departamento(&self, departamento: &String) -> Vec<&String> {
        let funcionarios = self.funcionarios.keys();

        let mut funcionarios_retorno: Vec<&String> = vec![];

        for funcionario in funcionarios {
            match self.funcionarios.get(funcionario) {
                Some(departamento_funcionario) => {
                    if departamento_funcionario.to_lowercase() == departamento.to_lowercase() {
                        funcionarios_retorno.push(funcionario)
                    }
                }
                None => continue,
            }
        }

        funcionarios_retorno
    }
    pub fn listar_funcionarios(&self) -> Vec<Vec<&String>> {
        self.funcionarios
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
            .group_by(|(_, valor)| *valor)
            .into_iter()
            .map(|(_, grupo)| grupo.map(|(chave, _)| chave).collect())
            .collect()
    }
}
