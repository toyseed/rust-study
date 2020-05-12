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

use crate::NodeVariant::{Nil, Node};
use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::rc::Rc;

struct DoubleListNode {
    value: String,
    prev: RefCell<Rc<NodeVariant>>,
    next: RefCell<Rc<NodeVariant>>,
}

impl DoubleListNode {
    fn new(value: &str) -> Self {
        DoubleListNode {
            value: String::from(value),
            prev: RefCell::new(Rc::new(Nil)),
            next: RefCell::new(Rc::new(Nil)),
        }
    }
}

enum NodeVariant {
    Node(DoubleListNode),
    Nil,
}

fn insert(head: Rc<NodeVariant>, target: &str, value: &str) {
    match head.as_ref() {
        Node(current) => {
            if &current.value != target {
                insert(Rc::clone(&current.next.borrow()), target, value);
                return;
            }

            let new_node = DoubleListNode::new(value);

            // new_node.next = current.next
            // new_node.prev = current
            *new_node.next.borrow_mut() = Rc::clone(&current.next.borrow());
            *new_node.prev.borrow_mut() = Rc::clone(&head);

            // current.next.prev = new_node
            // current.next = new_node
            let p_new_node = Rc::new(Node(new_node));
            if let Node(next) = current.next.borrow().as_ref() {
                *next.prev.borrow_mut() = Rc::clone(&p_new_node);
            }
            *current.next.borrow_mut() = Rc::clone(&p_new_node);
        }
        Nil => {
            return;
        }
    }
}

fn delete(head: Rc<NodeVariant>, target: &str) {
    match head.as_ref() {
        Node(current) => {
            if &current.value != target {
                delete(Rc::clone(&current.next.borrow()), target);
                return;
            }

            // current.prev.next = current.next
            // current.next.prev = current.prev
            if let Node(prev) = current.prev.borrow().as_ref() {
                *prev.next.borrow_mut() = Rc::clone(&current.next.borrow());
                println!("prev: {}", prev.value);
            }

            if let Node(next) = current.next.borrow().as_ref() {
                *next.prev.borrow_mut() = Rc::clone(&current.prev.borrow());
                println!("next: {}", next.value);
            }
        },
        Nil => {
            return;
        }
    }
}

fn modify(head: Rc<NodeVariant>, target: &str, value: &str) {
    let mut current = head;
    loop {
        let (targetNode, tryNext) = match current.as_ref() {
            Node(node) => {
                if &node.value != target {
                    (Some(Rc::new(node)), true)
                } else {
                    (Some(Rc::new(node)), false)
                }
            },
            Nil => {
                (None, false)
            }
        };

        if tryNext == true {
            if let Some(r_node) = targetNode {
                current = Rc::clone(&node.next.borrow());
            }
            continue;
        }

        if let Some(node) = targetNode {
            // to something
            break;
        }

        if tryNext == false {
            break;
        }

        // move current to next
    }
}

fn print_nodes(head: &NodeVariant, index: u32) {
    match head {
        Node(node) => {
            println!("{}: {}", index, node.value);

            print_nodes(node.next.borrow().as_ref(), index + 1);
        }
        Nil => {
            return;
        }
    }
}

fn main() {
    let a = Rc::new(Node(DoubleListNode::new("a")));
    insert(Rc::clone(&a), "a", "c");
    insert(Rc::clone(&a), "a", "b");
    insert(Rc::clone(&a), "c", "e");
    insert(Rc::clone(&a), "c", "d");

    print_nodes(a.as_ref(), 0);
    insert(Rc::clone(&a), "c", "z");
    print_nodes(a.as_ref(), 0);
    // delete(Rc::clone(&a), "b");
    // print_nodes(a.as_ref(), 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_tail(head: Rc<NodeVariant>) -> Rc<NodeVariant> {
        let variant = head.as_ref();

        match variant {
            Node(node) => {
                match node.next.borrow().as_ref() {
                    Node(node) => {
                        find_tail(Rc::clone(&node.next.borrow()))
                    },
                    Nil => {
                        Rc::clone(&head)
                    }
                }
            },
            Nil => {
                Rc::clone(&head)
            }
        }
    }

    fn print_from_tail(tail: &NodeVariant) {
        let mut current = tail;

        match current {
            Node(node) => {
                println!("{}", node.value);
                print_from_tail(node.prev.borrow().as_ref());
            }
            Nil => {
                return;
            }
        }
    }

    #[test]
    fn test_prev() {
        let a = Rc::new(Node(DoubleListNode::new("a")));
        insert(Rc::clone(&a), "a", "c");
        insert(Rc::clone(&a), "a", "b");
        insert(Rc::clone(&a), "c", "e");
        insert(Rc::clone(&a), "c", "d");

        let tail = find_tail(Rc::clone(&a));
        print_from_tail(tail.as_ref());
    }
}
