#[derive(Debug)]
enum DataBox {
    Something,
    Nothing,
}

fn main() {
    let c = DataBox::Something;
    println!("c = {:?}", c);
}
