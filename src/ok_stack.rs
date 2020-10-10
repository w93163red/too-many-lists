pub struct List<T> {
    head: Link<T>,
    size: i32,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_new() {
        let l = List::<i32>::new();
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

    #[test]
    fn test_peek() {
        let mut l = List::new();
        l.push(1);
        assert_eq!(l.peek(), Some(&1));
    }

    #[test]
    fn test_peek_mut() {
        let mut l = List::new();
        l.push(1);
        // match l.peek_mut() {
        //     Some(value) => {
        //         *value = 42;
        //     }
        //     _ => {}
        // }

        l.peek_mut().map(|value| {
            *value = 42;
        });

        // wrong case - l.peek_mut() still equals 1
        // let mut a = l.peek_mut();
        // a = Some(&mut 42);
        // correct use
        // a.map(|value| *value = 42);
        assert_eq!(l.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn test_into_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);

        let mut iter = l.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);

        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);

        let mut iter = l.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
