use std::collections::VecDeque;

#[derive(Debug)]
struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(vertices:usize) -> Self {
        Graph {
            vertices,
            adj_list: vec![Vec::new(); vertices],
        }
    }
    fn add_edge(&mut self, v: usize, w: usize) {
        self.adj_list[v].push(w);
        self.adj_list[w].push(v);
    }
    fn dfs_util(&self, v: usize, visited: &mut Vec<bool>) {
        visited[v] = true;
        println!("Visitando vertice: {}", v);
        for &neighbor in &self.adj_list[v] {
            if !visited[neighbor] {
                self.dfs_util(neighbor, visited);

            }
        }
    }
    fn dfs(&self, start: usize) {
        let mut visited = vec![false; self.vertices];
        println!("DFS a partir do vertice: {}", start);
        self.dfs_util(start, &mut visited);
    }    
    fn bfs(&self, start: usize) {
        let mut visited = vec![false; self.vertices];
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        println!("BFS a partir do vertices: {}", start);
        while let Some(v) = queue.pop_front() {
            println!("Visitando vertice: {}", v);
            for &neighbor in &self.adj_list[v] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }    
}
fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    graph.dfs(0);
    println!("============================");
    graph.bfs(0);
}
