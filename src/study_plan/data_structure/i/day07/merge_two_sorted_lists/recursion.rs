use super::list_node::ListNode;
use std::mem::take;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;

    if list1.is_none() {
        list2
    } else if list2.is_none() {
        list1
    } else if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
        list1.as_mut().unwrap().next =
            merge_two_lists(take(&mut list1.as_mut().unwrap().next), list2);
        list1
    } else {
        list2.as_mut().unwrap().next =
            merge_two_lists(take(&mut list2.as_mut().unwrap().next), list1);
        list2
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

    fn test<const L: usize, const M: usize, const N: usize>(
        list1: [i32; L],
        list2: [i32; M],
        output: [i32; N],
    ) {
        assert_eq!(
            super::merge_two_lists(array_to_list(list1), array_to_list(list2)),
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
