
fn main() {
    let v:Vec<i32> = (0..10).map(|x| x * x).collect();
    println!("{:?}", v);
}
