// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::{marker::PhantomData, ptr};

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
    _marker: PhantomData<T>,
}

type Link<T> = Option<ptr::NonNull<Node<T>>>;

impl<T> Node<T> {
    fn new(value: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            value,
            prev: None,
            next: None,
            _marker: PhantomData,
        }))
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _marker: PhantomData<T>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Link<T>,
}

pub struct Iter<'a, T> {
    current: Link<T>,
    _marker: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
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
        let head = self.head;

        Cursor {
            list: self,
            current: head,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let tail = self.tail;

        Cursor {
            list: self,
            current: tail,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_ptr = self.head;

        while let Some(node) = current_ptr {
            unsafe {
                let next_ptr = (*node.as_ptr()).next;
                drop(Box::from_raw(node.as_ptr()));
                current_ptr = next_ptr;
            }
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.current.map(|mut node| &mut node.as_mut().value) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(node) = self.current
                && let Some(next) = (*node.as_ptr()).next
            {
                self.current = (*node.as_ptr()).next;
                return Some(&mut (*next.as_ptr()).value);
            }

            None
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(node) = self.current
                && let Some(prev) = (*node.as_ptr()).prev
            {
                self.current = (*node.as_ptr()).prev;
                return Some(&mut (*prev.as_ptr()).value);
            }

            None
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unsafe {
            if let Some(current) = self.current {
                match (
                    (*current.as_ptr()).next.is_none(),
                    (*current.as_ptr()).prev.is_none(),
                    // (next is null, prev is null)
                ) {
                    (true, true) => {
                        // current is head AND tail
                        self.list.head = None;
                        self.list.tail = None;
                    }
                    (true, false) => {
                        // current is tail
                        let prev = (*current.as_ptr()).prev;

                        self.list.tail = prev;

                        if let Some(prev) = prev {
                            (*prev.as_ptr()).next = None;
                        }
                    }
                    (false, true) => {
                        // current is head

                        let next = (*current.as_ptr()).next;

                        self.list.head = next;

                        if let Some(next) = next {
                            (*next.as_ptr()).prev = None;
                        }
                    }
                    (false, false) => {
                        // current has both neighbors and is neither head nor tail

                        if let Some(prev) = (*current.as_ptr()).prev
                            && let Some(next) = (*current.as_ptr()).next
                        {
                            (*prev.as_ptr()).next = (*current.as_ptr()).next;
                            (*next.as_ptr()).prev = (*current.as_ptr()).prev;
                        }
                    }
                }

                let current_value = ptr::read(&(*current.as_ptr()).value);

                // set new self.current
                let new_current = if (*current.as_ptr()).next.is_none() {
                    (*current.as_ptr()).prev
                } else {
                    (*current.as_ptr()).next
                };

                drop(Box::from_raw(current.as_ptr()));

                self.current = new_current;

                self.list.len -= 1;

                return Some(current_value);
            }

            None
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node_ptr = ptr::NonNull::new(Node::new(element));

        unsafe {
            let Some(new_node) = new_node_ptr else {
                panic!()
            };

            if let Some(current) = self.current {
                if let Some(next) = (*current.as_ptr()).next {
                    // current is not tail
                    (*next.as_ptr()).prev = new_node_ptr;
                    (*new_node.as_ptr()).next = (*current.as_ptr()).next;
                    (*current.as_ptr()).next = new_node_ptr;
                    (*new_node.as_ptr()).prev = self.current;
                } else {
                    // current is tail
                    (*current.as_ptr()).next = new_node_ptr;
                    (*new_node.as_ptr()).prev = self.current;
                    self.list.tail = new_node_ptr;
                }
            } else {
                // list is empty
                self.list.head = new_node_ptr;
                self.list.tail = new_node_ptr;
                self.current = new_node_ptr;
            }
        }

        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new_node_ptr = ptr::NonNull::new(Node::new(element));

        unsafe {
            let Some(new_node) = new_node_ptr else {
                panic!()
            };

            if let Some(current) = self.current {
                if let Some(prev) = (*current.as_ptr()).prev {
                    // current is not head
                    (*prev.as_ptr()).next = new_node_ptr;
                    (*new_node.as_ptr()).prev = (*current.as_ptr()).prev;
                    (*current.as_ptr()).prev = new_node_ptr;
                    (*new_node.as_ptr()).next = self.current;
                } else {
                    // current is head
                    (*current.as_ptr()).prev = new_node_ptr;
                    (*new_node.as_ptr()).next = self.current;
                    self.list.head = new_node_ptr;
                }
            } else {
                // list is empty
                self.list.head = new_node_ptr;
                self.list.tail = new_node_ptr;
                self.current = new_node_ptr;
            }
        }

        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            let value = self.current.as_ref().map(|node| &(*node.as_ptr()).value);

            if let Some(current) = self.current {
                self.current = (*current.as_ptr()).next;
            }

            value
        }
    }
}
