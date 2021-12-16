
fn main() {
    let s = "abbcdde";
    
    let mut n = 0;

    for (c1, c2) in s.chars().zip(s.chars().skip(1)) {
        if c1 == c2 {
            n += 1;
        }
    }

    println!("{}", n);


}
