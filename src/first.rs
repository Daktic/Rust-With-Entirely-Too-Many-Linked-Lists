use std::mem;

pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem:i32) {
        let new_node = Box::new(Node {
            elem,
            // This works in the book, but needs a different implementation.
            // Possible I also missed something in the book.
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match &self.head {
            Link::Empty => {
              result = None;
            }
            Link::More(node) => {
                self.head = node.next;
            }
        };
        unimplemented!("TODO")
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}