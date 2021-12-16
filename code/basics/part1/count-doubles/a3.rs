
fn main() {
    let s = "abbcdde";
    
    let n:i32 = s.chars()
                .zip(s.chars().skip(1))
                .map(|(c1, c2)| if c1 == c2 {1} else {0})
                .sum();

    println!("{}", n);
}
