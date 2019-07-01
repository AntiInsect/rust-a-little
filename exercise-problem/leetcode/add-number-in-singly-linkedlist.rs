// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut ret = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut ret;
        loop {
            // the singly linked list can be seen an iterator which means it can use the method "take()"
            let adder_one = l1.take().unwrap_or(Box::new(ListNode::new(0)));
            let adder_two = l2.take().unwrap_or(Box::new(ListNode::new(0)));

            // as_mut : convert &mut Option<T> -> Option<&mut T>
            tail = match tail.as_mut() {
                Some(smart_node) => {
                    // adder_one, adder_two, smart_node auto unwrap
                    let sum = adder_one.val + adder_two.val + smart_node.val;
                    let carry = sum / 10;
                    smart_node.val = sum % 10;
                    if adder_one.next.is_none() && adder_two.next.is_none() && carry == 0 {
                        break ret;
                    } else {
                        smart_node.next = Some(Box::new(ListNode::new(carry)))
                    };
                    &mut smart_node.next
                }

                // better than using just a "{}"
                _ => unreachable!(),
            };
            l1 = adder_one.next;
            l2 = adder_two.next;
        }
    }
}