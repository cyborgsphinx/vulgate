use vulgate::{adder, multiplier, constant};
use vulgate::propagate::Connector;

fn main() {
    let c = Connector::new();
    let f = Connector::new();

    let u = Connector::new();
    let v = Connector::new();
    let w = Connector::new();
    let x = Connector::new();
    let y = Connector::new();

    multiplier!(c * w = u);
    multiplier!(v * x = u);
    adder!(v + y = f);
    constant!(w <- 9);
    constant!(x <- 5);
    constant!(y <- 32);

    let _ = c.borrow().set_value(25, 0);
    println!("f = {}", f.borrow().get_value().unwrap());
}
