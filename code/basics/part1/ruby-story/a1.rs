
fn main() {
    let v = vec![1,2,3,4];
    
    let f = |&x| x > 5;
   
    let p = v.iter().any(f);

    println!("{}", p);
}
