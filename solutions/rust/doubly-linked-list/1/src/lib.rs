// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::ptr;

struct Node<T> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    position: Option<usize>,
}

pub struct Iter<'a, T> {
    list: &'a LinkedList<T>,
    next_node: Option<&'a Node<T>>,
    initial_call: bool,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let position = if self.is_empty() { None } else { Some(0) };

        Cursor {
            list: self,
            position,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let position = if self.is_empty() {
            None
        } else {
            Some(self.len() - 1)
        };

        Cursor {
            list: self,
            position,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            next_node: None,
            initial_call: true,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        loop {
            if self.pop_front().is_none() {
                break;
            }
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.current_node().map(|node| {
                &mut node
                    .as_mut()
                    .expect("should be able to get a mutable reference to node")
                    .value
            })
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if self.position.is_some() && self.position.unwrap() < self.list.len() - 1 {
                self.position = self.position.map(|pos| pos + 1);
            }

            if let Some(current_node) = self.current_node()
                && let Some(element) = current_node.as_mut()
            {
                return Some(&mut element.value);
            }

            None
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if self.position.is_some() && self.position.unwrap() > 0 {
                self.position = self.position.map(|pos| pos - 1);
            }

            if let Some(current_node) = self.current_node()
                && let Some(element) = current_node.as_mut()
            {
                return Some(&mut element.value);
            }

            None
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        // SAFETY: TODO
        unsafe {
            if let Some(current_node) = self.current_node() {
                let mut removed_tail = false;

                if (*current_node).next.is_null() {
                    // current_node is the tail, set new one

                    self.list.tail = (*current_node).prev;
                    removed_tail = true;
                } else {
                    (*(*current_node).next).prev = (*current_node).prev;
                }

                if (*current_node).prev.is_null() {
                    // current_node is the head, set new one

                    self.list.head = (*current_node).next;
                } else {
                    (*(*current_node).prev).next = (*current_node).next;
                }

                self.list.len -= 1;

                if self.list.is_empty() {
                    self.position = None;
                } else if removed_tail {
                    self.position = Some(self.position.expect("position should be some value") - 1);
                }

                let current_node = Box::from_raw(current_node);
                return Some(current_node.value);
            }
        }

        None
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node = Node::new(element);

        unsafe {
            if let Some(current_node) = self.current_node() {
                if (*current_node).next.is_null() {
                    // We are at the tail of the list

                    (*current_node).next = new_node;
                    self.list.tail = new_node;
                    (*new_node).prev = current_node;
                } else {
                    // There is another node after this current node

                    (*new_node).next = (*current_node).next;
                    (*new_node).prev = current_node;
                    (*(*current_node).next).prev = new_node;
                    (*current_node).next = new_node;
                }
            } else {
                self.list.head = new_node;
                self.list.tail = new_node;
                self.position = Some(0);
            }
        }

        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new_node = Node::new(element);

        unsafe {
            if let Some(current_node) = self.current_node() {
                if (*current_node).prev.is_null() {
                    // We are at the head of the list

                    (*current_node).prev = new_node;
                    self.list.head = new_node;
                    (*new_node).next = current_node;
                } else {
                    // There is another node before this current node

                    (*new_node).prev = (*current_node).prev;
                    (*new_node).next = current_node;
                    (*(*current_node).prev).next = new_node;
                    (*current_node).prev = new_node;
                }

                self.position = self.position.map(|pos| pos + 1);
            } else {
                self.list.head = new_node;
                self.list.tail = new_node;
                self.position = Some(0);
            }
        }

        self.list.len += 1;
    }

    /// Returns `Some(*mut Node<T>)` at which the cursor is currently positioned if it is a valid position,
    /// `None` otherwise
    /// If `None` is returned, it is safe to assume that the list is empty.
    unsafe fn current_node(&mut self) -> Option<*mut Node<T>> {
        if self.position.is_none() {
            assert!(self.list.is_empty());
            return None;
        }

        let cursor_position = self.position.expect("position should be some value");

        let mut idx = 0;

        let mut current_node = self.list.head;

        unsafe {
            while idx < cursor_position {
                current_node = (*current_node).next;
                idx += 1;
            }
        }

        Some(current_node)
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if self.initial_call {
                self.next_node = self.list.head.as_ref();
                self.initial_call = false;
            }

            let current_element = self.next_node.map(|node| &node.value);

            self.next_node = if let Some(node) = self.next_node
                && !node.next.is_null()
            {
                Some(&*node.next)
            } else {
                None
            };

            current_element
        }
    }
}
