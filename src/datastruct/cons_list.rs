use std::fmt::Debug;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum ConsList<T> {
    Nil,
    Cons(T, Rc<ConsList<T>>),
}

#[allow(dead_code)]
impl<T: Debug> ConsList<T> {
    fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: DoubleEndedIterator,
    {
        iter.into_iter()
            .rev()
            .fold(Self::Nil, |acc, item| Self::Cons(item, Rc::new(acc)))
    }

    fn head(&self) -> Option<&T> {
        match self {
            Self::Nil => None,
            Self::Cons(value, _) => Some(value),
        }
    }

    fn get(&self, idx: usize) -> Option<&T> {
        match self {
            Self::Nil => None,
            Self::Cons(value, next) => {
                if idx == 0 {
                    Some(value)
                } else {
                    next.get(idx - 1)
                }
            }
        }
    }

    fn prepend(self: &Rc<Self>, item: T) -> Self {
        let tail = Rc::new(self);
        Self::Cons(item, Rc::clone(&tail))
    }

    fn shift(&mut self) -> Option<T> {
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
    use std::rc::Rc;

    use super::ConsList::*;
    use crate::datastruct::cons_list::ConsList;

    #[test]
    fn contruct() {
        let c = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
        println!("{:?}", c);
    }

    #[test]
    fn new() {
        let expected = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
        let list = ConsList::new(vec![1, 2, 3]);
        assert_eq!(list, expected);
    }

    #[test]
    fn head() {
        let list = ConsList::new(vec![1, 2, 3]);
        let head = list.head();
        assert_eq!(*head.unwrap(), 1);
    }

    #[test]
    fn get() {
        let list = ConsList::new(vec![1, 2, 3]);
        let head = list.get(2);
        assert_eq!(*head.unwrap(), 3);
    }

    #[test]
    fn shift() {
        let mut list = ConsList::new(vec![1, 2, 3]);
        let head = list.shift();
        assert_eq!(head.unwrap(), 1);
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
        let list1 = Cons(99, Rc::clone(&rc_tail));
        let list2 = Cons(88, Rc::clone(&rc_tail));

        assert_eq!(list1, ConsList::new(vec![99, 1, 2, 3]));
        assert_eq!(list2, ConsList::new(vec![88, 1, 2, 3]));
    }

    #[test]
    fn prepend() {
        let tail = ConsList::new(vec![1, 2, 3]);
        let rc_tail = Rc::new(tail);
        let list1 = rc_tail.prepend(88);
        let list2 = rc_tail.prepend(99);

        assert_eq!(list1, ConsList::new(vec![88, 1, 2, 3]));
        assert_eq!(list2, ConsList::new(vec![99, 1, 2, 3]));
    }

    #[test]
    #[should_panic]
    fn shift_panic() {
        let tail = ConsList::new(vec![1, 2, 3]);
        let rc_tail = Rc::new(tail);
        let mut list1 = Cons(99, Rc::clone(&rc_tail));
        let _list2 = Cons(88, Rc::clone(&rc_tail));
        list1.shift();
    }
}
