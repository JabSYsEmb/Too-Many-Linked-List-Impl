pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    pub fn push(
        &mut self,
        item: T,
    ) {
        let new_node = Box::new(Node {
            data: item,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
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
        assert_eq!(None, mylist.pop());
    }

    #[test]
    fn char_works() {
        let mut mylist = List::new();
        mylist.push('a');
        mylist.push('b');
        mylist.push('c');
        assert_eq!(Some('c'), mylist.pop());
        assert_eq!(Some('b'), mylist.pop());
        assert_eq!(Some('a'), mylist.pop());
        assert_eq!(None, mylist.pop());
        assert_eq!(None, mylist.pop());
    }

    #[test]
    fn string_works() {
        let mut mylist = List::new();
        mylist.push("My");
        mylist.push("First");
        mylist.push("Name");
        assert_eq!(Some("Name"), mylist.pop());
        assert_eq!(Some("First"), mylist.pop());
        assert_eq!(Some("My"), mylist.pop());
        assert_eq!(None, mylist.pop());
        assert_eq!(None, mylist.pop());
    }

    #[test]
    fn peek_works() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn peek_mut_works() {
        let mut list = List::new();
        assert_eq!(list.peek_mut(), None);
        list.push(2);
        list.push(3);
        list.peek_mut().map(|value| *value = 42);
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
