use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::rc::Rc;

pub trait Propagate {
    /// The type of the underlying value
    type Type;

    /// Returns a value to be used for identification
    fn id(&self) -> usize;

    /// Requests that the object update its internal state with new information
    fn update(&self);

    /// Requests that the object forget its current value
    fn forget(&self);
}

struct ConnectorState<T> {
    value: Option<T>,
    informant: Option<usize>,
}

impl<T> ConnectorState<T> {
    fn update(&mut self, new_value: Option<T>, new_info: Option<usize>) {
        self.value = new_value;
        self.informant = new_info;
    }
}

pub struct Connector<'a, T: Copy + Display + PartialEq> {
    state: RefCell<ConnectorState<T>>,
    constraints: Vec<Rc<RefCell<dyn Propagate<Type=T> + 'a>>>,
}

impl<'a, T: Copy + Display + PartialEq> Connector<'a, T> {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            state: RefCell::new(ConnectorState {
                value: None,
                informant: None,
            }),
            constraints: vec![],
        }))
    }

    pub fn has_value(&self) -> bool {
        self.state.borrow().value.is_some()
    }

    pub fn get_value(&self) -> Option<T> {
        self.state.borrow().value
    }

    // may need this to instead take &self
    pub fn set_value(&self, value: T, informer: usize) -> Result<(), String> {
        if let Some(old_value) = self.get_value() {
            if old_value != value {
                return Err(format!("Contradiction: {} != {}", old_value, value));
            }
        } else {
            self.state.borrow_mut().update(Some(value), Some(informer));
            for c in self.constraints.iter().filter(|c| c.borrow().id() != informer) {
                c.borrow().update();
            }
        }
        Ok(())
    }

    pub fn forget_value(&self, informer: usize) {
        if Some(informer) == self.state.borrow().informant {
            self.state.borrow_mut().update(None, None);
            for c in self.constraints.iter().filter(|c| c.borrow().id() != informer) {
                c.borrow().forget();
            }
        }
    }

    pub fn connect<'b: 'a>(&mut self, constraint: Rc<RefCell<dyn Propagate<Type=T> + 'b>>) -> Result<(), String> {
        let cons = Rc::clone(&constraint);
        if !self.constraints.iter().any(|c| c.borrow().id() == cons.borrow().id()) {
            self.constraints.push(cons);
            Ok(())
        } else {
            Err("Constraint already present".to_string())
        }
    }
}
