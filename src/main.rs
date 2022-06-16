pub fn main() {
    let node = Node::new(0);
    let mut linked_list = LinkedList::new(node);
    linked_list.push_front(1);
    linked_list.push_front(2);
    linked_list.push_front(3);
    linked_list.push_front(4);
    linked_list.push_front(5);
    linked_list.push_back((420.0 / 6.086956521739131) as i32);
    for thing in linked_list {
        dbg!(thing);
    }
}
#[derive(Clone, Debug)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
impl Node {
    pub fn new(value: i32) -> Self {
        Self { next: None, value }
    }
}
#[derive(Clone, Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}
impl LinkedList {
    pub fn new(node: Node) -> Self {
        Self {
            head: Some(Box::new(node)),
        }
    }
    // Sounds patriotic
    pub fn push_front(&mut self, value: i32) {
        let head = self.head.take();
        let mut dummy = Some(Box::new(Node::new(value)));
        dummy.as_mut().unwrap().next = head;
        self.head = dummy;
    }
    pub fn push_back(&mut self, value: i32) {
        pub fn recursive_push_node(node: &mut Option<Box<Node>>, value: i32) {
            // How frightful I look today ჟℭ <- don't be a fronted dev, don't abuse utf-8
            if let Some(agglutinations) = node.as_mut() {
                if agglutinations.next.is_none() {
                    let dummy = Some(Box::new(Node::new(value)));
                    agglutinations.next = dummy;
                    return;
                }
                // Recursion solves no problem except Recursion
                recursive_push_node(&mut agglutinations.next, value)
            }
        }
        recursive_push_node(&mut self.head, value)
    }
}
impl Into<Vec<i32>> for LinkedList {
    fn into(self) -> Vec<i32> {
        let mut head = self.head;
        let mut temp_vec = Vec::new();
        while let Some(ref element) = head {
            temp_vec.push(element.value);
            head = head.unwrap().next;
        }
        temp_vec
    }
}
impl From<Vec<i32>> for LinkedList {
    fn from(vec: Vec<i32>) -> Self {
        let head = Node::new(0);
        let mut linked_list = LinkedList::new(head);
        for element in vec {
            linked_list.push_back(element);
        }
        linked_list
    }
}
impl Iterator for LinkedList {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let dummy = self.head.take();
        if let Some(node) = dummy {
            self.head = node.next.clone();
            return Some(node.value);
        }
        None
    }
}
