fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Use safe indexing
    println!("First element: {}", v[0]);
}

//Alternative using iter_mut
fn main() {
    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        *i = *i *2; //Safe modification using iter_mut
    }
    println!("Modified vector: {:?}",v);
}