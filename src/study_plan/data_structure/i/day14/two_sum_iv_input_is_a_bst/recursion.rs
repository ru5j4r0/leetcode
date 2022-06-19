use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    if root.is_none() {
        return false;
    }
    _find_target(Rc::clone(root.as_ref().unwrap()), k, &mut HashSet::new())
}

fn check_child(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>) -> bool {
    if let Some(n) = node {
        _find_target(Rc::clone(n), k, set)
    } else {
        false
    }
}

fn _find_target(node: Rc<RefCell<TreeNode>>, k: i32, set: &mut HashSet<i32>) -> bool {
    let n = node.borrow();
    let val = n.val;

    if set.contains(&val) {
        return true;
    }
    set.insert(k - val);

    check_child(&n.left, k, set) || check_child(&n.right, k, set)
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

    fn test<const N: usize>(root: [Option<i32>; N], k: i32, res: bool) {
        assert_eq!(find_target(array_to_tree(root), k), res);
    }

    #[test]
    fn case1() {
        test(
            [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
            9,
            true,
        );
    }

    #[test]
    fn case2() {
        test(
            [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
            28,
            false,
        );
    }
}
