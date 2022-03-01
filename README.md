```rust
fn main() {
    let mut g = Graph::new(10);
    // add_edge(src, dst, weight)
    g.add_edge(0, 1, 5 as usize);
    g.add_edge(0, 2, 3 as usize);
    g.add_edge(0, 3, 2 as usize);
    g.add_edge(0, 5, 2 as usize);
    g.add_edge(1, 3, 1 as usize);
    g.add_edge(2, 4, 1 as usize);
    g.add_edge(3, 4, 1 as usize);
    g.add_edge(4, 5, 3 as usize);

    let result = g.dijkstra(0);
    for i in 0..10 {
        print!("{}, {{Distance: {:?}, ", i, result.get_distance(i));

        print!("Path: ");
        if let Some(path) = result.get_path(i) {
            for j in 0..path.len() - 1 {
                print!("{} -> ", path[j]);
            }
            print!("{}", path[path.len() - 1]);
        } else {
            print!("The path does not exist.");
        }
        println!("}}");
    }
    // 0, {Distance: Some(0), Path: 0}
    // 1, {Distance: Some(5), Path: 0 -> 1}
    // 2, {Distance: Some(3), Path: 0 -> 2}
    // 3, {Distance: Some(2), Path: 0 -> 3}
    // 4, {Distance: Some(3), Path: 0 -> 3 -> 4}
    // 5, {Distance: Some(2), Path: 0 -> 5}
    // 6, {Distance: None, Path: The path does not exist.}
    // 7, {Distance: None, Path: The path does not exist.}
    // 8, {Distance: None, Path: The path does not exist.}
    // 9, {Distance: None, Path: The path does not exist.}
}
```