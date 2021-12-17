#[derive(Debug)]
enum DataBox {
    Something(i32),
    Nothing,
}

fn main() {
    let c = DataBox::Something(23);
    println!("c = {:?}", c);
    let d = DataBox::Nothing;
    println!("d = {:?}", d);
}
