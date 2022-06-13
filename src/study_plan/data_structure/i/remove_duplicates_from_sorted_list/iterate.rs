use super::list_node::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut head = head;
    let mut curr = head.as_mut().unwrap();

    while let Some(next) = curr.next.as_mut() {
        if curr.val == next.val {
            curr.next = next.next.take();
        } else {
            curr = curr.next.as_mut().unwrap();
        }
    }

    head
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
        assert_eq!(
            super::delete_duplicates(array_to_list(list)),
            array_to_list(res)
        );
    }

    #[test]
    fn case1() {
        test([1, 1, 2], [1, 2]);
    }

    #[test]
    fn case2() {
        test([1, 1, 2, 3, 3], [1, 2, 3]);
    }
}
