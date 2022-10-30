//! This is adapted from
//! https://github.com/rust-unofficial/too-many-lists/blob/bec3afe0c33ff2bdce6895126055e4c5fa0dbd7d/lists/src/second.rs

#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>,
}

type BoxedNode<T> = Box<Node<T>>;
type Link<T> = Option<BoxedNode<T>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let old_head: Link<T> = self.head.take();

        let node = Node {
            elem,
            next: old_head,
        };

        let boxed_node: BoxedNode<T> = Box::new(node);
        let new_head: Link<T> = Some(boxed_node);
        self.head = new_head;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head: Link<T> = self.head.take();

        match old_head {
            None => None,
            Some(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                let elem = node.elem;
                Some(elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let head = &self.head;

        match head {
            None => None,
            Some(boxed_node) => {
                let elem = &boxed_node.elem;
                Some(elem)
            }
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let head = &mut self.head;

        match head {
            None => None,
            Some(boxed_node) => {
                let elem = &mut boxed_node.elem;
                Some(elem)
            }
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link: Link<T> = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T> {
    stack: Stack<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let mut stack = Stack::new();

        // Check empty list behaves right
        assert_eq!(stack.pop(), None);

        // Populate list
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        match stack.peek_mut() {
            None => (),
            Some(value) => {
                *value = 42;
            }
        };

        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
    }

    #[test]
    fn test_into_iter() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
