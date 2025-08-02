use std::iter::FromIterator;

pub struct SimpleLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
}

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        if let Some(node) = &self.head {
            let mut curr = node;

            len += 1;

            while let Some(node) = &curr.next {
                curr = node;

                len += 1;
            }
        }

        len
    }

    pub fn push(&mut self, _element: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node {
                data: _element,
                next: None,
            }));
        } else {
            let old_head = self.head.take();

            let new_head = Some(Box::new(Node {
                data: _element,
                next: old_head,
            }));

            self.head = new_head;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();

        if let Some(node) = old_head {
            self.head = node.next;
            return Some(node.data);
        }

        None
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref()?;

        self.head.as_ref().map(|n| &n.as_ref().data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();

        let mut curr = &self.head;

        while let Some(ref node) = curr {
            reversed.push((*node).clone().data);
            curr = &node.next
        }

        reversed
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        _iter.into_iter().for_each(|element| {
            list.push(element);
        });

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();

        while let Some(data) = _linked_list.pop() {
            vec.push(data);
        }

        vec.reverse();

        vec
    }
}
