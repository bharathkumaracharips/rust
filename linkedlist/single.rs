use std::fmt;

// Node definition
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// List definition
pub struct SinglyLinkedList {
    head: Option<Box<Node>>,
}

impl SinglyLinkedList {
    // Create new empty list
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    // Insert at front
    pub fn push_front(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove from front
    pub fn pop_front(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    // Display the list
    pub fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

// Optional: Implement Debug trait
impl fmt::Debug for SinglyLinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.data));
            current = &node.next;
        }
        result.push_str("None");
        write!(f, "{}", result)
    }
}

// Test the list
fn main() {
    let mut list = SinglyLinkedList::new();
    list.push_front(10);
    list.push_front(20);
    list.push_front(30);

    println!("Current List:");
    list.print_list();

    println!("Popped: {:?}", list.pop_front());
    println!("After pop:");
    list.print_list();
}