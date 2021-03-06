use super::list_node::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list = None;
    let mut node = &mut list;
    let mut node1 = list1;
    let mut node2 = list2;

    while node1.is_some() && node2.is_some() {
        if node1.as_ref().unwrap().val <= node2.as_ref().unwrap().val {
            *node = node1;
            node1 = node.as_mut().unwrap().next.take();
        } else {
            *node = node2;
            node2 = node.as_mut().unwrap().next.take();
        }
        node = &mut node.as_mut().unwrap().next;
    }

    *node = if node1.is_some() { node1 } else { node2 };

    list
}

#[cfg(test)]
mod test {
    use super::*;

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

    fn test<const L: usize, const M: usize, const N: usize>(
        list1: [i32; L],
        list2: [i32; M],
        output: [i32; N],
    ) {
        assert_eq!(
            merge_two_lists(array_to_list(list1), array_to_list(list2)),
            array_to_list(output)
        );
    }

    #[test]
    fn case1() {
        test([1, 2, 4], [1, 3, 4], [1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn case2() {
        test([], [], []);
    }

    #[test]
    fn case3() {
        test([], [0], [0]);
    }
}
