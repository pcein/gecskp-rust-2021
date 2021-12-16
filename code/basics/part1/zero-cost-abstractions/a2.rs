
fn main() {
    let v:Vec<i32> = (0..10).map(|x| x + 1).collect();
    println!("{:?}", v);
}
