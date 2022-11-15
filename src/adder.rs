use crate::propagate::{Connector, Propagate};

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::rc::{Rc, Weak};

pub struct Adder<'a, T>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy + Display + PartialEq + 'a,
{
    a1: Weak<RefCell<Connector<'a, T>>>,
    a2: Weak<RefCell<Connector<'a, T>>>,
    sum: Weak<RefCell<Connector<'a, T>>>,
}

impl<'a, T> Adder<'a, T>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy + Display + PartialEq + 'a,
{
    pub fn new(
        a1: Rc<RefCell<Connector<'a, T>>>,
        a2: Rc<RefCell<Connector<'a, T>>>,
        sum: Rc<RefCell<Connector<'a, T>>>,
    ) -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            a1: Rc::downgrade(&a1),
            a2: Rc::downgrade(&a2),
            sum: Rc::downgrade(&sum),
        }));
        let _ = a1.borrow_mut().connect(Rc::clone(&this) as _);
        let _ = a2.borrow_mut().connect(Rc::clone(&this) as _);
        let _ = sum.borrow_mut().connect(Rc::clone(&this) as _);
        this
    }
}

impl<'a, T> Propagate for Adder<'a, T>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Copy + Display + PartialEq,
{
    type Type = T;

    fn id(&self) -> usize {
        // using pointer as id assumes the object doesn't move between calls
        self as *const Self as usize
    }

    // need to rethink borrows; currently recursively borrowing, which is causing failures
    fn update(&self) {
        let Some(a1) = self.a1.upgrade() else {
            eprintln!("a1 not present");
            return;
        };
        let Some(a2) = self.a2.upgrade() else {
            eprintln!("a2 not present");
            return;
        };
        let Some(sum) = self.sum.upgrade() else {
            eprintln!("sum not present");
            return;
        };
        let a1_value = a1.borrow().get_value();
        let a2_value = a2.borrow().get_value();
        let sum_value = sum.borrow().get_value();
        let result = match (a1_value, a2_value, sum_value) {
            (Some(v1), Some(v2), None) => sum.borrow().set_value(v1 + v2, self.id()),
            (Some(v1), None, Some(total)) => a2.borrow().set_value(total - v1, self.id()),
            (None, Some(v2), Some(total)) => a1.borrow().set_value(total - v2, self.id()),
            _ => Ok(()),
        };
        if let Err(msg) = result {
            println!("{msg}");
        }
    }

    fn forget(&self) {
        if let Some(a1) = self.a1.upgrade() {
            a1.borrow().forget_value(self.id());
        } else {
            eprintln!("a1 not present");
        }
        if let Some(a2) = self.a2.upgrade() {
            a2.borrow().forget_value(self.id());
        } else {
            eprintln!("a2 not present");
        }
        if let Some(sum) = self.sum.upgrade() {
            sum.borrow().forget_value(self.id());
        } else {
            eprintln!("sum not present");
        }
    }
}

#[macro_export]
macro_rules! adder {
    ($a1:ident + $a2:ident = $sum:ident) => {
        ::gensym::gensym! { $crate::adder!($a1, $a2, $sum) }
    };
    ($guard:ident, $a1:ident, $a2:ident, $sum:ident) => {
        let $guard = $crate::adder::Adder::new(
            ::std::rc::Rc::clone(&$a1),
            ::std::rc::Rc::clone(&$a2),
            ::std::rc::Rc::clone(&$sum),
        );
    };
}
