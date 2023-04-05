use std::ptr::null_mut;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            // Hello Compiler, I Know I Am Doing Something Dangerous And
            // I Promise To Be A Good Programmer Who Never Makes Mistakes.
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    // pub fn pop(&mut self) -> Option<T> {
    //     self.head.take().map(|head| {
    //         let head = *head;
    //         self.head = head.next;

    //         if self.head.is_none() {
    //             self.tail = None;
    //         }

    //         head.elem
    //     })
    // }
}

#[cfg(test)]
mod test {
    // use super::List;
    // #[test]
    // fn basics() {
    // let mut list = List::new();

    // // Check empty list behaves right
    // assert_eq!(list.pop(), None);

    // // Populate list
    // list.push(1);
    // list.push(2);
    // list.push(3);

    // // Check normal removal
    // assert_eq!(list.pop(), Some(1));
    // assert_eq!(list.pop(), Some(2));

    // // Push some more just to make sure nothing's corrupted
    // list.push(4);
    // list.push(5);

    // // Check normal removal
    // assert_eq!(list.pop(), Some(4));

    // // Check exhaustion
    // assert_eq!(list.pop(), Some(5));
    // assert_eq!(list.pop(), None);
    // }
}
