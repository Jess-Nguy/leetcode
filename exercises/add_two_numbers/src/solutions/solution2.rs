use crate::ListNode;

pub struct Solution2 {}

/// This is a reference to a posted solution for learning purpose - https://leetcode.com/Qba101/
impl Solution2 {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution2::add_two_numbers(n1.next, n2.next),
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution2::add_two_numbers(
                            Solution2::add_two_numbers(carry, n1.next),
                            n2.next,
                        ),
                    }))
                }
            }
        }
    }
}
