use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use std::time::Instant;
    #[test]
    fn t() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    #[test]
    fn tt() {
        let mut arr = [0u8; 32768];

        for data in &mut arr {
            *data = rand::random_range(0..255);
        }
        arr.sort();

        let start = Instant::now();

        let mut sum = 0u64;
        for _ in 0u32..100000u32 {
            for &data in &arr {
                if data >= 128 {
                    sum += data as u64;
                }
            }
        }

        let duration = start.elapsed();

        println!("sum: {:?}", sum);
        println!("耗时: {:?}", duration);
        println!("毫秒: {} ms", duration.as_millis());
        println!("微秒: {} μs", duration.as_micros());
        println!("纳秒: {} ns", duration.as_nanos());
    }
}
