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
            elem,
            prev: None,
            next: None,
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

    // Is this where I put this?
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // Non-empty list. Need to connect the old_head to the new_head
                old_head.prev = Some(new_head.clone());
                new_head.next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                // Empty list. Need to set the tail
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}