use vulgate::adder::Adder;
use vulgate::constant::Constant;
use vulgate::multiplier::Multiplier;
use vulgate::propagate::Connector;

use std::rc::Rc;

fn main() {
    let c = Connector::new();
    let f = Connector::new();

    let u = Connector::new();
    let v = Connector::new();
    let w = Connector::new();
    let x = Connector::new();
    let y = Connector::new();

    let _formula1 = Multiplier::new(Rc::clone(&c), /* * */ Rc::clone(&w), /* = */ Rc::clone(&u));
    let _formula2 = Multiplier::new(Rc::clone(&v), /* * */ Rc::clone(&x), /* = */ Rc::clone(&u));
    let _formula3 = Adder::new(Rc::clone(&v), /* + */ Rc::clone(&y), /* = */ Rc::clone(&f));
    let _formula4 = Constant::new(9, /* -> */ Rc::clone(&w));
    let _formula5 = Constant::new(5, /* -> */ Rc::clone(&x));
    let _formula6 = Constant::new(32, /* -> */ Rc::clone(&y));

    let _ = c.borrow().set_value(25, 0);
    println!("f = {}", f.borrow().get_value().unwrap());
}
