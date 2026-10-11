use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum ConsList<T> {
    // With `Rc`, you can put both the value and the list on the heap.
    // `RefCell` allows you to modify the inner value without a mutable reference to the list.
    Cons(Rc<RefCell<T>>, Rc<ConsList<T>>),
    Nil,
}

#[allow(dead_code)]
impl<T: Debug> ConsList<T> {
    fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: DoubleEndedIterator,
    {
        iter.into_iter().rev().fold(Self::Nil, |acc, item| {
            Self::Cons(Rc::new(RefCell::new(item)), Rc::new(acc))
        })
    }

    fn head(&self) -> Option<Rc<RefCell<T>>> {
        match self {
            Self::Nil => None,
            Self::Cons(value, _) => Some(Rc::clone(value)),
        }
    }

    fn get(&self, idx: usize) -> Option<Rc<RefCell<T>>> {
        match self {
            Self::Nil => None,
            Self::Cons(value, next) => {
                if idx == 0 {
                    Some(Rc::clone(value))
                } else {
                    next.get(idx - 1)
                }
            }
        }
    }

    fn prepend(self: Rc<Self>, item: Rc<RefCell<T>>) -> Self {
        Self::Cons(item, Rc::clone(&self))
    }

    fn shift(&mut self) -> Option<Rc<RefCell<T>>> {
        match std::mem::replace(self, Self::Nil) {
            Self::Nil => None,
            Self::Cons(value, next) => match Rc::try_unwrap(next) {
                Ok(next) => {
                    *self = next;
                    Some(value)
                }
                Err(next) => {
                    panic!("cannot shift a shared tail: {:?}", next);
                }
            },
        }
    }

    fn rev(mut self) -> Self {
        let mut result = Self::Nil;
        while let Some(value) = self.shift() {
            result = Self::Cons(value, Rc::new(result));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::ConsList::*;
    use super::ConsList;

    #[test]
    fn new() {
        let expected = Cons(
            Rc::new(RefCell::new(1)),
            Rc::new(Cons(
                Rc::new(RefCell::new(2)),
                Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
            )),
        );
        let list = ConsList::new(vec![1, 2, 3]);
        assert_eq!(list, expected);
    }

    #[test]
    fn head() {
        let list = ConsList::new(vec![1, 2, 3]);
        let head = list.head();
        assert_eq!(*head.unwrap().borrow(), 1);
    }

    #[test]
    fn get() {
        let list = ConsList::new(vec![1, 2, 3]);
        let head = list.get(2);
        assert_eq!(*head.unwrap().borrow(), 3);
    }

    #[test]
    fn shift() {
        let mut list = ConsList::new(vec![1, 2, 3]);
        let head = list.shift();
        assert_eq!(*head.unwrap().borrow(), 1);
        assert_eq!(list, ConsList::new(vec![2, 3]));
    }

    #[test]
    fn rev() {
        let list = ConsList::new(vec![1, 2, 3]);
        let rev = list.rev();
        assert_eq!(rev, ConsList::new(vec![3, 2, 1]));
    }

    #[test]
    fn share_tail() {
        let tail = ConsList::new(vec![1, 2, 3]);
        let rc_tail = Rc::new(tail);
        let list1 = Cons(Rc::new(RefCell::new(88)), Rc::clone(&rc_tail));
        let list2 = Cons(Rc::new(RefCell::new(99)), Rc::clone(&rc_tail));

        assert_eq!(list1, ConsList::new(vec![88, 1, 2, 3]));
        assert_eq!(list2, ConsList::new(vec![99, 1, 2, 3]));
    }

    #[test]
    fn prepend() {
        let tail = Rc::new(ConsList::new(vec![1, 2, 3]));
        let list1 = Rc::clone(&tail).prepend(Rc::new(RefCell::new(88)));
        let list2 = Rc::clone(&tail).prepend(Rc::new(RefCell::new(99)));

        assert_eq!(list1, ConsList::new(vec![88, 1, 2, 3]));
        assert_eq!(list2, ConsList::new(vec![99, 1, 2, 3]));
    }

    #[test]
    #[should_panic]
    fn panic_when_shift_with_share_tail() {
        let tail = Rc::new(ConsList::new(vec![1, 2, 3]));
        let mut list1 = Cons(Rc::new(RefCell::new(88)), Rc::clone(&tail));
        let _list2 = Cons(Rc::new(RefCell::new(99)), Rc::clone(&tail));
        list1.shift();
    }

    #[test]
    fn change_val_when_share_tail() {
        let val = Rc::new(RefCell::new(88));

        let list1 = Cons(Rc::clone(&val), Rc::new(Nil));
        assert_eq!(list1, ConsList::new(vec![88]));
        let list2 = Cons(Rc::new(RefCell::new(99)), Rc::new(list1));
        assert_eq!(list2, ConsList::new(vec![99, 88]));

        *Rc::clone(&val).borrow_mut() = 100;
        assert_eq!(list2, ConsList::new(vec![99, 100]));
    }
}
