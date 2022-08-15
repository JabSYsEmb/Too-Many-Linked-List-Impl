pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    data: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    pub fn push(
        &mut self,
        item: i32,
    ) {
        let new_node = Box::new(Node {
            data: item,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut next_node) = cur_link {
            cur_link = next_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let empty_list = List::new();
    }

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
