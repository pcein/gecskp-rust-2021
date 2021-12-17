fn sum(x:i32, y:i32) -> i32 {
    x + y
}

fn main() {
    let mut v = vec![1,2,3,4];
    
    let a = v.pop();
    let b = v.pop();

    let s = sum(a, b);

    println!("sum = {}", s);
}
