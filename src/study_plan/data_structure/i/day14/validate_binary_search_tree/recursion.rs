use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    _is_valid_bst(Rc::clone(root.as_ref().unwrap()), None, None)
}

fn check_child(node: &Rc<RefCell<TreeNode>>, min: Option<i32>, max: Option<i32>) -> bool {
    let val = node.borrow().val;
    let low_cond = if let Some(low) = min { low < val } else { true };
    let high_cond = if let Some(high) = max {
        val < high
    } else {
        true
    };
    if low_cond && high_cond {
        _is_valid_bst(Rc::clone(node), min, max)
    } else {
        false
    }
}

fn _is_valid_bst(node: Rc<RefCell<TreeNode>>, min: Option<i32>, max: Option<i32>) -> bool {
    let n = node.borrow();

    let is_valid_left = if let Some(l) = &n.left {
        check_child(l, min, Some(n.val))
    } else {
        true
    };

    let is_valid_right = if let Some(r) = &n.right {
        check_child(r, Some(n.val), max)
    } else {
        true
    };

    is_valid_left && is_valid_right
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

    fn test<const N: usize>(root: [Option<i32>; N], res: bool) {
        assert_eq!(is_valid_bst(array_to_tree(root)), res);
    }

    #[test]
    fn case1() {
        test([Some(2), Some(1), Some(3)], true);
    }

    #[test]
    fn case2() {
        test(
            [Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
            false,
        );
    }
}
