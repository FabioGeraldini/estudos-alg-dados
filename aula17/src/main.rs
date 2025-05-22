use std::collections::{VecDeque};

fn main() { 
    //Exemplo 3: VecDeque - Fila de deque(double-ended queue)
    let mut deque = VecDeque::new();
    deque.push_back(10);
    deque.push_back(20);
    deque.push_back(5);
    println!("VecDeque: {:?}", deque);
    // Remove Elemento do Início
    if let Some(front) = deque.pop_front() {
        println!("Elemento removido do início: {}", front);
    }
}    
