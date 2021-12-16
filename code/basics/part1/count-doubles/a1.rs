
fn main() {
    let s = "abbcdde";
    
    let v:Vec<(char, char)> = s.chars().zip(s.chars().skip(1)).collect();

    let mut n = 0;

    for (c1, c2) in v {
        if c1 == c2 {
            n += 1;
        }
    }

    println!("{}", n);


}
