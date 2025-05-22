use std::collections::{HashMap};

fn main() {
    // Exemplo 1: Hasmap - Estrutura de dados assosciativa (dicionário)
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"),75);
    scores.insert(String::from("Carol"), 85);
    println!("HashMap (scores): {:?}", scores);
    // Intera sobre as chaves e valores
    for (name, scores) in &scores {
        println!("{}: {}", name, scores);
    }
}
