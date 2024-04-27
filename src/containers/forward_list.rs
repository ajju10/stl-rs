use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element,
            next: None,
        }
    }
}

pub struct ForwardList<T> {
    head: Option<NonNull<Node<T>>>,
    size: usize,
}

impl<T> ForwardList<T> {
    pub fn new() -> Self {
        ForwardList {
            head: None,
            size: 0,
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().element) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().element) }
    }

    pub fn push_back(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        let ptr = NonNull::from(Box::leak(node));
        self.push_back_node_ptr(ptr);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node_ptr().map(|node| node.element)
    }
}

impl<T> Default for ForwardList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for ForwardList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.head;
        let mut fmt_str = String::new();
        while let Some(node_ptr) = temp {
            unsafe {
                fmt_str.push_str(&format!("{:?}->", (*node_ptr.as_ptr()).element));
                temp = (*node_ptr.as_ptr()).next;
            }
        }

        fmt_str.push_str("None");
        write!(f, "{fmt_str}")
    }
}

// Private Methods
impl<T> ForwardList<T> {
    fn push_back_node_ptr(&mut self, ptr: NonNull<Node<T>>) {
        let mut temp = self.head;
        let mut temp1 = None;
        if temp.is_none() {
            self.head = Some(ptr);
        } else {
            while let Some(node_ptr) = temp {
                unsafe {
                    temp1 = temp;
                    temp = (*node_ptr.as_ptr()).next;
                }
            }

            let node_ptr = temp1.unwrap().as_ptr();
            unsafe {
                (*node_ptr).next = Some(ptr);
            }
        }

        self.size += 1;
    }

    fn pop_back_node_ptr(&mut self) -> Option<Box<Node<T>>> {
        if self.head.is_none() {
            return None;
        }

        let mut temp = self.head;
        let mut temp1 = None;

        loop {
            if temp.is_some() {
                let node_ptr = temp.unwrap();
                unsafe {
                    if (*node_ptr.as_ptr()).next.is_none() {
                        break;
                    } else {
                        temp1 = temp;
                        temp = (*node_ptr.as_ptr()).next;
                    }
                }
            }
        }

        temp.map(|node_ptr| unsafe {
            let node = Box::from_raw(node_ptr.as_ptr());
            if temp1.is_none() {
                self.head = None;
            } else {
                let prev_ptr = temp1.unwrap();
                (*prev_ptr.as_ptr()).next = None;
            }
            self.size -= 1;
            node
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut list = ForwardList::new();
        list.push_back(10);
        list.push_back(20);
        assert_eq!(list.size, 2);
        assert!(list.head.is_some());
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.size, 1);
        assert_eq!(list.front(), Some(&10));

        match list.front_mut() {
            None => {}
            Some(val) => *val = 20,
        }

        assert_eq!(list.front(), Some(&20));

        list.pop_back();
        assert!(list.head.is_none());
        assert_eq!(list.size, 0);
        assert_eq!(list.front(), None);
    }
}
