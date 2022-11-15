use crate::propagate::{Propagate, Connector};

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::rc::Rc;

pub struct Adder<'a, T> where T: Add<T, Output=T> + Sub<T, Output=T> + Copy + Display + PartialEq + 'a {
    a1: Rc<RefCell<Connector<'a, T>>>,
    a2: Rc<RefCell<Connector<'a, T>>>,
    sum: Rc<RefCell<Connector<'a, T>>>,
}

impl<'a, T> Adder<'a, T> where T: Add<T, Output=T> + Sub<T, Output=T> + Copy + Display + PartialEq + 'a {
    pub fn new(a1: Rc<RefCell<Connector<'a, T>>>, a2: Rc<RefCell<Connector<'a, T>>>, sum: Rc<RefCell<Connector<'a, T>>>) -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            a1: Rc::clone(&a1),
            a2: Rc::clone(&a2),
            sum: Rc::clone(&sum),
        }));
        let _ = a1.borrow_mut().connect(Rc::clone(&this) as _);
        let _ = a2.borrow_mut().connect(Rc::clone(&this) as _);
        let _ = sum.borrow_mut().connect(Rc::clone(&this) as _);
        this
    }
}

impl<'a, T> Propagate for Adder<'a, T> where T: Add<T, Output=T> + Sub<T, Output=T> + Copy + Display + PartialEq {
    type Type = T;

    fn id(&self) -> usize {
        // using pointer as id assumes the object doesn't move between calls
        self as *const Self as usize
    }

    // need to rethink borrows; currently recursively borrowing, which is causing failures
    fn update(&mut self) {
        let Ok(mut a1) = self.a1.try_borrow_mut() else {
            eprintln!("Could not borrow a1 in update");
            return;
        };
        let Ok(mut a2) = self.a2.try_borrow_mut() else {
            eprintln!("Could not borrow a2 in update");
            return;
        };
        let Ok(mut sum) = self.sum.try_borrow_mut() else {
            eprintln!("Could not borrow sum in update");
            return;
        };
        let result = match (a1.get_value(), a2.get_value(), sum.get_value()) {
            (Some(v1), Some(v2), None) => sum.set_value(v1 + v2, self.id()),
            (Some(v1), None, Some(total)) => a2.set_value(total - v1, self.id()),
            (None, Some(v2), Some(total)) => a1.set_value(total - v2, self.id()),
            (..) => Ok(()),
        };
        if let Err(msg) = result {
            println!("{msg}");
        }
    }

    fn forget(&mut self) {
        if self.a1.try_borrow_mut().map(|mut c| c.forget_value(self.id())).is_err() {
            eprintln!("Could not borrow a1 in forget");
        }
        if self.a2.try_borrow_mut().map(|mut c| c.forget_value(self.id())).is_err() {
            eprintln!("Could not borrow a2 in forget");
        }
        if self.sum.try_borrow_mut().map(|mut c| c.forget_value(self.id())).is_err() {
            eprintln!("Could not borrow sum in forget");
        }
    }
}
