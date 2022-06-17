use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::mem::replace;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.as_ref()?;
    _invert_tree(Rc::clone(root.as_ref().unwrap()));
    root
}

fn check_child(node: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(n) = node {
        _invert_tree(Rc::clone(n));
    }
}

fn _invert_tree(node: Rc<RefCell<TreeNode>>) {
    if node.borrow().left.is_some() || node.borrow().right.is_some() {
        let left = node.borrow_mut().left.take();
        let right = replace(&mut node.borrow_mut().right, left);
        node.borrow_mut().left = right;

        check_child(&node.borrow().left);
        check_child(&node.borrow().right);
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

    fn test<const M: usize, const N: usize>(root: [Option<i32>; M], res: [Option<i32>; N]) {
        assert_eq!(invert_tree(array_to_tree(root)), array_to_tree(res));
    }

    #[test]
    fn case1() {
        test(
            [
                Some(4),
                Some(2),
                Some(7),
                Some(1),
                Some(3),
                Some(6),
                Some(9),
            ],
            [
                Some(4),
                Some(7),
                Some(2),
                Some(9),
                Some(6),
                Some(3),
                Some(1),
            ],
        );
    }

    #[test]
    fn case2() {
        test([Some(2), Some(1), Some(3)], [Some(2), Some(3), Some(1)]);
    }

    #[test]
    fn case3() {
        test([], []);
    }
}
