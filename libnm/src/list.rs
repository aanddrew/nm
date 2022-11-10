use std::{rc::Rc, sync::Arc, fmt};

use crate::program::Item;

pub struct List <T> {
    head: Link<T>
}

type Link<T> = Option<Arc<Node<T>>>;

pub struct Node <T> {
    car: T,
    next: Link<T>
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Arc::new(Node {
            car: elem,
            next: self.head.clone(),
        }))}
    }

    /*
    fn concat_backwards(self, other: List<T>) -> List<&T> {
        match other.car() {
            Some(elem) => {
                let ret = self.concat(other.cdr());
                ret.prepend(elem)
            },
            _ => other
        }
    }

    pub fn concat(self, other: List<T>) -> List<T> {
        other.concat_backwards(self)
    }
    */

    pub fn cdr(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn car(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.car)
    }

    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter { next: self.head.as_deref() }
    }
}

impl <T> Clone for List<T> {
    fn clone(&self) -> Self {
        Self { head: self.head.clone() }
    }
}

pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.car
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            }
            else {
                break;
            }
        }
    }
}

impl fmt::Debug for List<Item> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = if self.cdr().car().is_none() { "" } else { " " };
        match self.car() {
            Some(item) => f.write_str(format!("{:?}{}{:?}", item, separator, self.cdr()).as_str()),
            _ => f.write_str(")"),
        }
        //f.debug_struct("List").field("head", &self.head).finish()
    }
}

impl fmt::Debug for Node<Item> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.car {
            Item::List(list) => match &list.head { 
                Some(node) => f.debug_struct("List").field("head", &node).finish(),
                None => f.write_str(")")
            },
            _ => f.debug_struct("Node").field("car", &self.car).field("next", &self.next).finish()
        }
    }
}