use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                                            // total: +2 new_head, +0 old_head -- OK!
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
                                            // total: +2 new_head -- OK!
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // -1 new
                    // not emptying list
                    new_head.borrow_mut().prev.take(); // -1 old
                    self.head = Some(new_head); // +1 new
                                                // total: -2 old, +0 new
                }
                None => {
                    // emptying list
                    self.tail.take(); // -1 old
                                      // total: -2 old, (no new)
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}
