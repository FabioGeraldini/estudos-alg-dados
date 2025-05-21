#[derive(Debug)]
struct HashTable { 
    buckets: Vec<Vec<(String, i32)>>,
    size: usize,
}

impl HashTable {
    fn new(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(Vec::new());
        }
        HashTable { buckets, size }
    }
    fn simple_hash(&self, key: &String) -> usize {
        let sum: usize = key.bytes().map(|b| b as usize).sum();
        sum % self.size
        }
    fn insert(&mut self, key: String, value: i32) {
        let index = self.simple_hash(&key);
        for entry in self.buckets[index].iter_mut(){
            if entry.0 == key {
                entry.1 = value;
                return;
            }
        }
        self.buckets[index].push((key, value));
    }
    fn get(&self, key: &String) -> Option<i32> {
        let index = self.simple_hash(key);
        for entry in &self.buckets[index] {
            if &entry.0 == key {
                return Some(entry.1);
            }
        }
        None
    }
    }

fn main(){
    let mut table = HashTable::new(10);
    table.insert("chave1". to_string(), 100);
    table.insert("chave2". to_string(), 200);
    table.insert("outra_chave". to_string(), 300);
    println!("Valor para 'chave1': {:?}", table.get(&"chave1".to_string()));
    println!("Valor para 'chave2': {:?}", table.get(&"chave1".to_string()));
    println!("Valor para 'outra_chave': {:?}", table.get(&"outra_chave".to_string()));
    println!("Valor para 'nao_existe': {:?}", table.get(&"nao_existe".to_string()));
}
