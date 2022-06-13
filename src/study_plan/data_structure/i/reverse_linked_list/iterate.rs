use super::list_node::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr = head;
    let mut prev = None;
    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
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

    fn test<const M: usize, const N: usize>(list: [i32; M], res: [i32; N]) {
        assert_eq!(super::reverse_list(array_to_list(list)), array_to_list(res));
    }

    #[test]
    fn case1() {
        test([1, 2, 3, 4, 5], [5, 4, 3, 2, 1]);
    }

    #[test]
    fn case2() {
        test([1, 2], [2, 1]);
    }

    #[test]
    fn case3() {
        test([], []);
    }
}
