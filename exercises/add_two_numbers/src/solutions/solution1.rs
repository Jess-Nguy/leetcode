use crate::ListNode;

pub struct Solution1;

impl Solution1 {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution1::add_two_numbers_recursive(l1, l2, 0)
    }

    fn add_two_numbers_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                if carry == 0 {
                    None
                } else {
                    Some(Box::new(ListNode::new(carry)))
                }
            }
            (Some(l1), None) => {
                let sum = l1.val + carry;
                let carry = sum / 10;
                let val = sum % 10;
                Some(Box::new(ListNode {
                    val,
                    next: Self::add_two_numbers_recursive(l1.next, None, carry),
                }))
            }
            (None, Some(l2)) => {
                let sum = l2.val + carry;
                let carry = sum / 10;
                let val = sum % 10;
                Some(Box::new(ListNode {
                    val,
                    next: Self::add_two_numbers_recursive(None, l2.next, carry),
                }))
            }
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val + carry;
                let carry = sum / 10;
                let val = sum % 10;
                Some(Box::new(ListNode {
                    val,
                    next: Self::add_two_numbers_recursive(l1.next, l2.next, carry),
                }))
            }
        }
    }
}
