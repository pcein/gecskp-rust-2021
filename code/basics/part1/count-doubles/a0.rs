
fn main() {
    let s = "abbcdde";

    let v:Vec<(char, char)> = s.chars().zip(s.chars().skip(1)).collect();


    println!("{:?}", v);

}
