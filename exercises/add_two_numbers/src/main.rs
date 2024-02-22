mod solutions;
use solutions::solution1::Solution1;
use solutions::solution2::Solution2;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let solution = Solution1::add_two_numbers(
        Some(Box::new(ListNode::new(2))),
        Some(Box::new(ListNode::new(3))),
    );
    println!("Solution1 {:?}", solution);

    let solution = Solution2::add_two_numbers(
        Some(Box::new(ListNode::new(2))),
        Some(Box::new(ListNode::new(3))),
    );
    println!("Solution2 {:?}", solution);
}

#[cfg(test)]
mod solutions_tests {
    use super::*;

    /// Example 1: Input: l1 = [2,4,3], l2 = [5,6,4]
    /// Output: [7,0,8]
    /// Explanation: 342 + 465 = 807.
    #[test]
    fn test_add_two_numbers_carry_over() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));
        assert_eq!(Solution1::add_two_numbers(l1.clone(), l2.clone()), expected);
        assert_eq!(Solution2::add_two_numbers(l1, l2), expected);
    }

    /// Example 2: Input: l1 = [0], l2 = [0]
    /// Output: [0]
    /// Explanation: 0 + 0 = 0.
    #[test]
    fn test_add_two_numbers_0s() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));
        let expected = Some(Box::new(ListNode::new(0)));

        assert_eq!(Solution1::add_two_numbers(l1.clone(), l2.clone()), expected);
        assert_eq!(Solution2::add_two_numbers(l1, l2), expected);
    }

    /// Example 3: Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    /// Output: [8,9,9,9,0,0,0,1]
    /// Explanation: 9999999 + 9999 = 10009998.
    #[test]
    fn test_add_two_numbers_different_lengths() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode::new(9))),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode::new(9))),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode::new(1))),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution1::add_two_numbers(l1.clone(), l2.clone()), expected);
        assert_eq!(Solution2::add_two_numbers(l1, l2), expected);
    }
}
