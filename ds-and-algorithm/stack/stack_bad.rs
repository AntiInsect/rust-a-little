// origin code from https://rust-unofficial.github.io/too-many-lists/first-final.html
// modified by wangshsh2@shanghaitech.edu.cn

// WHY is the following bad ?
// 1. do not use Option but use self-writing enum Link which does the same thing.
// 2. do not use Rc<RefCell<>> but use Box, this does not make sense because we do not need to
// modify the inner struct such that the mutability from Box is not necessary.
// 3. mem::relpace is expensive
// 4. no generic

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
