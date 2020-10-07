use std::mem;

pub struct List {
    head: Link,
    size: i32,
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
        List {
            head: Link::Empty,
            size: 0,
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_new() {
        let l = List::new();
        assert_eq!(l.size, 0);
    }

    #[test]
    fn test_push() {
        let mut l = List::new();
        l.push(1);
        assert_eq!(l.size, 1);
        assert_eq!(l.pop(), Some(1));
    }

    #[test]
    fn test_pop() {
        let mut l = List::new();
        l.push(1);
        assert_eq!(l.size, 1);
        let v = l.pop();
        assert_eq!(l.size, 0);
        assert_eq!(v, Some(1));
    }
}
