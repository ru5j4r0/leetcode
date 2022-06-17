use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }

    _has_path_sum(Rc::clone(root.as_ref().unwrap()), target_sum, 0)
}

fn check_child(node: &Option<&Rc<RefCell<TreeNode>>>, target_sum: i32, current_sum: i32) -> bool {
    if let Some(n) = node {
        _has_path_sum(Rc::clone(n), target_sum, current_sum)
    } else {
        false
    }
}

fn _has_path_sum(node: Rc<RefCell<TreeNode>>, target_sum: i32, mut current_sum: i32) -> bool {
    current_sum += node.borrow().val;

    let node = node.borrow();
    let left = node.left.as_ref();
    let right = node.right.as_ref();

    if left.is_none() && right.is_none() {
        target_sum == current_sum
    } else {
        check_child(&left, target_sum, current_sum) || check_child(&right, target_sum, current_sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Ptr = Rc<RefCell<TreeNode>>;
    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn new_node(val: i32) -> Node {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn set_node(opt: &Option<i32>, node: &mut Node, queue: &mut VecDeque<Ptr>) {
        match opt {
            Some(val) => {
                *node = new_node(*val);
                queue.push_back(Rc::clone(node.as_ref().unwrap()));
            }
            None => {
                *node = None;
            }
        }
    }

    fn array_to_tree<const N: usize>(arr: [Option<i32>; N]) -> Node {
        if N == 0 {
            return None;
        }

        let head = new_node(arr[0].unwrap());
        let mut node = Rc::clone(&head.as_ref().unwrap());
        let mut queue = VecDeque::new();

        for chunk in arr[1..].chunks(2) {
            set_node(&chunk[0], &mut node.borrow_mut().left, &mut queue);
            if let Some(opt) = chunk.get(1) {
                set_node(opt, &mut node.borrow_mut().right, &mut queue);
            }
            node = queue.pop_front().unwrap();
        }

        head
    }

    fn test<const N: usize>(root: [Option<i32>; N], target_sum: i32, res: bool) {
        assert_eq!(has_path_sum(array_to_tree(root), target_sum), res);
    }

    #[test]
    fn case1() {
        test(
            [
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                None,
                Some(1),
            ],
            22,
            true,
        );
    }

    #[test]
    fn case2() {
        test([Some(1), Some(2), Some(3)], 5, false);
    }

    #[test]
    fn case3() {
        test([], 0, false);
    }
}
