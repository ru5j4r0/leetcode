use super::list_node::ListNode;

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut curr = &mut head;

    loop {
        match curr {
            Some(node) if node.val == val => {
                *curr = node.next.take();
            }
            Some(node) => {
                curr = &mut node.next;
            }
            None => {
                return head;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ListNode;

    fn array_to_list<const N: usize>(arr: [i32; N]) -> Option<Box<ListNode>> {
        if N == 0 {
            return None;
        }

        let mut head = Box::new(ListNode::new(arr[0]));
        let mut node = &mut head;

        for &val in arr.iter().skip(1) {
            node.next = Some(Box::new(ListNode::new(val)));
            node = node.next.as_mut().unwrap();
        }

        Some(head)
    }

    fn test<const M: usize, const N: usize>(head: [i32; M], val: i32, res: [i32; N]) {
        assert_eq!(
            super::remove_elements(array_to_list(head), val),
            array_to_list(res)
        );
    }

    #[test]
    fn case1() {
        test([1, 2, 6, 3, 4, 5, 6], 6, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn case2() {
        test([], 1, []);
    }

    #[test]
    fn case3() {
        test([7, 7, 7, 7], 7, []);
    }
}
