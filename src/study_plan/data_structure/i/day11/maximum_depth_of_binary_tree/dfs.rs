use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root.as_ref() {
        _max_depth(Rc::clone(node), 1)
    } else {
        0
    }
}

fn check_child(node: &Option<Rc<RefCell<TreeNode>>>, level: i32) -> i32 {
    if let Some(left) = node.as_ref() {
        _max_depth(Rc::clone(left), level + 1)
    } else {
        level
    }
}

fn _max_depth(node: Rc<RefCell<TreeNode>>, level: i32) -> i32 {
    max(
        check_child(&node.borrow().left, level),
        check_child(&node.borrow().right, level),
    )
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

    fn test<const N: usize>(root: [Option<i32>; N], res: i32) {
        assert_eq!(max_depth(array_to_tree(root)), res);
    }

    #[test]
    fn case1() {
        test(
            [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            3,
        );
    }

    #[test]
    fn case2() {
        test([Some(1), None, Some(2)], 2);
    }
}
