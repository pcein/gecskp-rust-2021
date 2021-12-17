fn sum(x:i32, y:i32) -> i32 {
    x + y
}

fn main() {
    let mut v = vec![1, 2];
    
    let a = v.pop();
    println!("a = {:?}", a);

    let b = v.pop();
    println!("b = {:?}", b);

    let c = v.pop();
    println!("c = {:?}", c);

}
