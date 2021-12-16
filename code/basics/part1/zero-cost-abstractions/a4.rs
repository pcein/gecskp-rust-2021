
fn main() {
    let v:u64 = (0..100000000).map(|x| x + 1).sum();
    println!("{:?}", v);
}
