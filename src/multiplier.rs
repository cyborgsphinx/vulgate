use crate::propagate::{Connector, Propagate};

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::Display;
use std::ops::{Div, Mul};
use std::rc::{Rc, Weak};

pub struct Multiplier<'a, T>
where
    T: Mul<T, Output = T> + Div<T, Output = T> + From<isize> + Copy + Display + PartialEq + 'a,
{
    m1: Weak<RefCell<Connector<'a, T>>>,
    m2: Weak<RefCell<Connector<'a, T>>>,
    product: Weak<RefCell<Connector<'a, T>>>,
}

impl<'a, T> Multiplier<'a, T>
where
    T: Mul<T, Output = T> + Div<T, Output = T> + From<isize> + Copy + Display + PartialEq + 'a,
{
    pub fn new(
        m1: Rc<RefCell<Connector<'a, T>>>,
        m2: Rc<RefCell<Connector<'a, T>>>,
        product: Rc<RefCell<Connector<'a, T>>>,
    ) -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            m1: Rc::downgrade(&m1),
            m2: Rc::downgrade(&m2),
            product: Rc::downgrade(&product),
        }));
        let _ = m1.borrow_mut().connect(Rc::clone(&this) as _);
        let _ = m2.borrow_mut().connect(Rc::clone(&this) as _);
        let _ = product.borrow_mut().connect(Rc::clone(&this) as _);
        this
    }
}

impl<'a, T> Propagate for Multiplier<'a, T>
where
    T: Mul<T, Output = T> + Div<T, Output = T> + From<isize> + Copy + Display + PartialEq,
{
    type Type = T;

    fn id(&self) -> usize {
        // using pointer as id assumes the object doesn't move between calls
        self as *const Self as usize
    }

    // need to rethink borrows; currently recursively borrowing, which is causing failures
    fn update(&self) {
        let Some(m1) = self.m1.upgrade() else {
            eprintln!("m1 not present");
            return;
        };
        let Some(m2) = self.m2.upgrade() else {
            eprintln!("m2 not present");
            return;
        };
        let Some(product) = self.product.upgrade() else {
            eprintln!("product not present");
            return;
        };
        let m1_value = m1.borrow().get_value();
        let m2_value = m2.borrow().get_value();
        let product_value = product.borrow().get_value();
        let zero: T = 0.into();
        let result = match (m1_value, m2_value, product_value) {
            (Some(val), _, _) | (_, Some(val), _) if val == zero => {
                product.borrow().set_value(zero, self.id())
            }
            (Some(v1), Some(v2), None) => product.borrow().set_value(v1 * v2, self.id()),
            (Some(v1), None, Some(result)) => m2.borrow().set_value(result / v1, self.id()),
            (None, Some(v2), Some(result)) => m1.borrow().set_value(result / v2, self.id()),
            _ => Ok(()),
        };
        if let Err(msg) = result {
            println!("{msg}");
        }
    }

    fn forget(&self) {
        if let Some(m1) = self.m1.upgrade() {
            m1.borrow().forget_value(self.id());
        } else {
            eprintln!("m1 not present");
        }
        if let Some(m2) = self.m2.upgrade() {
            m2.borrow().forget_value(self.id());
        } else {
            eprintln!("m2 not present");
        }
        if let Some(product) = self.product.upgrade() {
            product.borrow().forget_value(self.id());
        } else {
            eprintln!("product not present");
        }
    }
}

#[macro_export]
macro_rules! multiplier {
    ($a1:ident * $a2:ident = $sum:ident) => {
        ::gensym::gensym! { $crate::multiplier!($a1, $a2, $sum) }
    };
    ($guard:ident, $m1:ident, $m2:ident, $product:ident) => {
        let $guard = $crate::multiplier::Multiplier::new(
            ::std::rc::Rc::clone(&$m1),
            ::std::rc::Rc::clone(&$m2),
            ::std::rc::Rc::clone(&$product),
        );
    };
}
