//! This is adapted from
//! https://github.com/rust-unofficial/too-many-lists/blob/bec3afe0c33ff2bdce6895126055e4c5fa0dbd7d/lists/src/second.rs

pub struct Stack<T> {
    head: Link<T>,
}

type BoxedNode<T> = Box<Node<T>>;
type Link<T> = Option<BoxedNode<T>>;

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
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
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
}
