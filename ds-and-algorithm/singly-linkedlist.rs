use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
        }))
    }
}

#[derive(Clone)]
pub struct SinglyLinkedlist {
    head: Link,
    tail: Link,
    len: u64,
}

impl SinglyLinkedlist {
    pub fn new_empty() -> SinglyLinkedlist {
        SinglyLinkedlist {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> &u64 {
        &self.len
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            // the clone there are all very inexpensive
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        };
        self.len += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.len -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("WRONG")
                .into_inner()
                .value
        })
    }

    fn pop_without_return_inner_value(&mut self) {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else { self.tail.take(); }
            self.len -= 1;
        });
    }
}

