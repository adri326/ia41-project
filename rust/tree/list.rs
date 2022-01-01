use std::rc::Rc;

pub type RcNode<T> = Option<Rc<RcLinkedList<T>>>;

pub struct RcLinkedList<T: Clone> {
    pub value: T,
    pub parent: Option<Rc<RcLinkedList<T>>>,
}

impl<T: Clone> RcLinkedList<T> {
    pub fn new(value: T, parent: &Option<Rc<RcLinkedList<T>>>) -> Self {
        RcLinkedList {
            value,
            parent: parent.as_ref().map(|t| Rc::clone(t)),
        }
    }

    pub fn push(self: Rc<RcLinkedList<T>>, value: T) -> Self {
        RcLinkedList {
            value,
            parent: Some(Rc::clone(&self))
        }
    }

    pub fn into_iter_rc(self: Rc<RcLinkedList<T>>) -> RcLinkedListIter<T> {
        RcLinkedListIter {
            current: Some(self)
        }
    }
}

pub struct RcLinkedListIter<T: Clone> {
    current: Option<Rc<RcLinkedList<T>>>,
}

impl<T: Clone> Iterator for RcLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(list) => {
                let res = list.value.clone();
                self.current = list.parent.as_ref().map(|t| Rc::clone(t));
                Some(res)
            },
            None => None
        }
    }
}
