
fn main() {
    let mut v:Vec<i32> = vec![];
    let m = v.pop(); 
    match m {
        Some(x) => println!("{}", x),
        None    => println!("popped from empty list")
    }  
}
