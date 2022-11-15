use crate::propagate::{Propagate, Connector};

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::rc::{Rc, Weak};

pub struct Constant<'a, T> where T: Copy + Display + PartialEq + 'a {
    _conn: Weak<RefCell<Connector<'a, T>>>,
}

impl<'a, T> Constant<'a, T> where T: Copy + Display + PartialEq + 'a {
    pub fn new(value: T, connector: Rc<RefCell<Connector<'a, T>>>) -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            _conn: Rc::downgrade(&connector),
        }));
        let _ = connector.borrow_mut().connect(Rc::clone(&this) as _);
        let _ = connector.borrow().set_value(value, this.borrow().id());
        this
    }
}

impl<'a, T> Propagate for Constant<'a, T> where T: Copy + Display + PartialEq + 'a {
    type Type = T;

    fn id(&self) -> usize {
        self as *const Self as usize
    }

    fn update(&self) {
        // do nothing
    }

    fn forget(&self) {
        // do nothing
    }
}
