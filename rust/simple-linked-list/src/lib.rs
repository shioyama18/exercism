type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut ptr = self.head.as_ref();
        while let Some(node_ref) = ptr {
            count += 1;
            ptr = node_ref.next.as_ref();
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let new = Box::new(Node {
            data: element,
            next: self.head.take()
        });
        self.head = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut ptr = self.head.as_ref();
        while let Some(node) = ptr {
            list.push(node.data.clone());
            ptr = node.next.as_ref();
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in item.iter() {
            list.push(i.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(node) = self.pop() {
            vec.push(node);
        }
        vec.reverse();
        vec
    }
}
