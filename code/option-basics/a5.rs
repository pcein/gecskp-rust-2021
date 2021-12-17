fn main() {
    let mut v = vec![1,2,3,4];
    
    let a = v.pop();

    match a {
        Some(x) => println!("popped item: {}", x),
        None => println!("stack empty!"),
    }
}
