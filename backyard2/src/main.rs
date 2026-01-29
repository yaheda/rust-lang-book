use crate::garden::vegetables::Asparagus;
use crate::garden::fruits::Apple;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I planted {:?}!", plant);

    let fruit = Apple {};
    println!("I love {:?}!", fruit);
}
