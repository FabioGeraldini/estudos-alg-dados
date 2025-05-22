use std::collections::{BinaryHeap};

fn main() {
   //Exemplo 4: BinaryHeap - Heap Binário (fila de prioridade)
   let mut heap = BinaryHeap::new();
   heap.push(8);
   heap.push(3);
   heap.push(5);
   println!("BinaryHeap (max-heap): {:?}", heap);
   // O elemento no topo é maior
   if let Some(max) = heap.peek() {
    println!("Elemento máximo: {}", max);
   }
}
