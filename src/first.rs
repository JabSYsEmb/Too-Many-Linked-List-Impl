use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    data: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
        }
    }

    pub fn push(
        &mut self,
        item: i32,
    ) {
        let new_node = Box::new(Node {
            data: item,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.data)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut next_node) = cur_link {
            cur_link = mem::replace(&mut next_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_works() {
        let mut mylist = List::new();
        mylist.push(4);
        mylist.push(5);
    }

    #[test]
    fn pop_works() {
        let mut mylist = List::new();
        mylist.push(1);
        mylist.push(2);
        mylist.push(3);
        assert_eq!(Some(3), mylist.pop());
        assert_eq!(Some(2), mylist.pop());
        assert_eq!(Some(1), mylist.pop());
        assert_eq!(None, mylist.pop());
    }
}
