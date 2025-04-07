/*!
# Add Two Numbers
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

**Example 1:**
```
Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807
```

**Example 2:**
```
Input: l1 = [0], l2 = [0]
Output: [0]
```

**Example 2:**
```
Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]
```

Constraints:
- The number of nodes in each linked list is in the range [1, 100].
- 0 <= Node.val <= 9
- It is guaranteed that the list represents a number that does not have leading zeros.

*/

pub mod solution {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }
    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode { val, next: None }
        }
    }
    fn add_two(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                if let 0 = carry {
                    None
                } else {
                    Some(Box::new(ListNode::new(carry)))
                }
            }
            (None, Some(node2)) => add_two(Some(node2), None, carry),
            (Some(mut node1), None) => {
                let sum = node1.val + carry;
                node1.val = sum % 10;
                node1.next = add_two(node1.next.take(), None, sum / 10);
                Some(node1)
            }
            (Some(mut node1), Some(mut node2)) => {
                let sum = node1.val + node2.val;
                node1.val = sum % 10;
                node1.next = add_two(node1.next.take(), node2.next.take(), sum / 10);
                Some(node1)
            }
        }
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        add_two(l1, l2, 0)
    }
}
