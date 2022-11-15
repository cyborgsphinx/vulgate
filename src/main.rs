use vulgate::adder::Adder;
use vulgate::propagate::Connector;

use std::rc::Rc;

fn main() {
    let y = Connector::new();
    let mx = Connector::new();
    let b = Connector::new();

    let _formula = Adder::new(Rc::clone(&mx), Rc::clone(&b), Rc::clone(&y));

    let _ = y.borrow().set_value(10, 0);
    let _ = b.borrow().set_value(4, 0);
    println!("mx = {}", mx.borrow().get_value().unwrap()); // currently panics
}
