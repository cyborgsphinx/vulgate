use vulgate::adder::Adder;
use vulgate::propagate::Connector;

use std::rc::Rc;

fn main() {
    let y = Connector::new();
    let mx = Connector::new();
    let b = Connector::new();

    let _formula = Adder::new(Rc::clone(&mx), Rc::clone(&b), Rc::clone(&y));

    let _ = y.borrow_mut().set_value(10, 0);
    let _ = b.borrow_mut().set_value(4, 0);
    assert!(mx.borrow().get_value().is_some()); // currently panics
}
