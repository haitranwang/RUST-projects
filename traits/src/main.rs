mod basket;

use basket::Basket;

fn main() {
    let b1 = Basket::new(String::from("hi there"));

    let b2 = Basket::new(10);

    let b3 = Basket::new(true);
}
