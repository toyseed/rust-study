/*
1. struct DoubleListNode {
    value: String,
    prev: "앞 노드로의 포인터",
    next: "뒤 노드로의 포인터",
}
이중 연결 링크드 리스트를 만들어서 3~5개의 노드로 구성된 리스트를 만드세요.
이 리스트에 삽입(특정 value의 뒤에 삽입), 삭제(특정 value 노드 삭제),
수정(특정 value값을 가진 노드 모두 변경), 리스트 전체 출력을 하는 함수를 작성하세요.
이 때 메모리 누수가 되지 않게 주의하세요.
 */

use std::rc::Rc;
use crate::NodeVariant::{Node, Nil};
use std::cell::RefCell;

struct DoubleLinkedNode {
    value: RefCell<String>,
    prev: RefCell<Rc<NodeVariant>>,
    next: RefCell<Rc<NodeVariant>>,
}

impl DoubleLinkedNode {
    fn new(value: &str) -> Self {
        DoubleLinkedNode {
            value: RefCell::new(String::from(value)),
            prev: RefCell::new(Rc::new(Nil)),
            next: RefCell::new(Rc::new(Nil)),
        }
    }

    fn set_prev(&self, node: Rc<NodeVariant>) {
        *self.prev.borrow_mut() = Rc::clone(&node);
    }

    fn set_next(&self, node: Rc<NodeVariant>) {
        *self.next.borrow_mut() = Rc::clone(&node);
    }
}

enum NodeVariant {
    Node(DoubleLinkedNode),
    Nil,
}

struct DoubleLinkedList {
    head: RefCell<Rc<NodeVariant>>,
}

impl DoubleLinkedList {
    fn new(value: &str) -> Self {
        DoubleLinkedList {
            head: RefCell::new(Rc::new(Node(DoubleLinkedNode::new(value)))),
        }
    }

    fn insert_to_after(&self, target: &str, value: &str) {
        let mut current_variant = Rc::clone(&self.head.borrow());

        loop {
            let maybe_next_variant: Option<Rc<NodeVariant>> = match current_variant.as_ref() {
                Node(current_node) => {
                    if current_node.value.borrow().as_str() != target {
                        Some(Rc::clone(&current_node.next.borrow()))
                    } else {
                        let new_node = DoubleLinkedNode::new(value);

                        new_node.set_prev(Rc::clone(&current_variant));
                        new_node.set_next(Rc::clone(&current_node.next.borrow()));

                        let new_node_ref = Rc::new(Node(new_node));

                        if let Node(next_node) = current_node.next.borrow().as_ref() {
                            next_node.set_prev(Rc::clone(&new_node_ref));
                        }

                        current_node.set_next(Rc::clone(&new_node_ref));
                        None
                    }
                },
                Nil => {
                    None
                }
            };

            match maybe_next_variant {
                Some(next_variant) => {
                    current_variant = next_variant;
                }
                None => { break; }
            }
        }
    }

    fn delete(&self, target: &str) {
        let mut current_variant = Rc::clone(&self.head.borrow());

        loop {
            let maybe_next_variant = match current_variant.as_ref() {
                Node(current_node) => {
                    if current_node.value.borrow().as_str() != target {
                        Some(Rc::clone(&current_node.next.borrow()))
                    } else {
                        let prev_variant = Rc::clone(&current_node.prev.borrow());
                        let next_variant = Rc::clone(&current_node.next.borrow());

                        if let Node(prev_node) = prev_variant.as_ref() {
                            prev_node.set_next(Rc::clone(&next_variant));
                        } else {
                            *self.head.borrow_mut() = Rc::clone(&next_variant);
                        }

                        if let Node(next_node) = next_variant.as_ref() {
                            next_node.set_prev(Rc::clone(&prev_variant));
                        }

                        None
                    }
                },
                Nil => {
                    None
                }
            };

            match maybe_next_variant {
                Some(next_variant) => {
                    current_variant = next_variant;
                },
                None => { break; }
            }
        }
    }

    fn modify(&self, target: &str, value: &str) {
        let mut current_variant = Rc::clone(&self.head.borrow());

        loop {
            let maybe_next_variant = match current_variant.as_ref() {
                Node(current_node) => {
                    if current_node.value.borrow().as_str() == target {
                        *current_node.value.borrow_mut() = String::from(value);
                    }

                    Some(Rc::clone(&current_node.next.borrow()))
                },
                Nil => {
                    None
                }
            };

            match maybe_next_variant {
                Some(next_variant) => {
                    current_variant = next_variant;
                },
                None => { break; }
            }
        }
    }

    fn print(&self) {
        let mut current_variant = Rc::clone(&self.head.borrow());
        let mut index = 0;

        loop {
            let maybe_next_variant = match current_variant.as_ref() {
                Node(current_node) => {
                    index = index + 1;
                    print!("{}) {} ", index, current_node.value.borrow().as_str());
                    Some(Rc::clone(&current_node.next.borrow()))
                },
                Nil => {
                    None
                }
            };

            match maybe_next_variant {
                Some(next_variant) => {
                    current_variant = next_variant;
                }
                None => { break; }
            }
        }

        println!();
    }
}

fn main() {
    let list = DoubleLinkedList::new("z");
    list.insert_to_after("z", "a");
    list.insert_to_after("a", "b");
    list.insert_to_after("b", "d");
    list.insert_to_after("b", "c");
    list.insert_to_after("d", "z");
    println!("init list");
    list.print();

    list.delete("z");
    list.delete("z");
    println!("after deleting 'z'");
    list.print();

    list.insert_to_after("a", "z");
    list.insert_to_after("b", "z");
    list.insert_to_after("c", "z");
    println!("after inserting 'z'");
    list.print();

    list.modify("z", "!");
    println!("after modifying 'z' to '!'");
    list.print();
}

