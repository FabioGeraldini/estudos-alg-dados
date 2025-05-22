use std::collections::{HashSet};
 fn main() {
    // Exemplo 2: Hashset - Conjunto de valores únicos
    let mut unique_numbers = HashSet::new();
    unique_numbers.insert(1);
    unique_numbers.insert(2);
    unique_numbers.insert(3);
    unique_numbers.insert(2);
    println!("Hashset (número únicos): {:?}", unique_numbers);
 }
