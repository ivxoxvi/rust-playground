use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Node<T> {
    value: T,
    parent: Rc<RefCell<Option<Node<T>>>>,
    children: Vec<Rc<Node<T>>>,
}

#[allow(dead_code)]
impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            parent: Rc::new(RefCell::new(None)),
            children: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::{cell::RefCell, rc::Rc};

    #[test]
    fn new() {
        let child = Rc::new(Node::new(2));
        let mut parent = Node::new(1);
        parent.children.push(Rc::clone(&child));
        // let rc_parent = Rc::new(RefCell::new(Some(parent)));
        *child.parent.borrow_mut() = Some(parent);
        println!("{:?}",child);
        // println!("{:?}",parent);
    }
}
