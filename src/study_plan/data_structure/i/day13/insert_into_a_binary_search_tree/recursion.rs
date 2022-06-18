use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return new_node(val);
    }

    _insert_into_bst(Rc::clone(root.as_ref().unwrap()), val);

    root
}

fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

fn check_child(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(n) = node {
        _insert_into_bst(Rc::clone(n), val);
    } else {
        *node = new_node(val);
    }
}

fn _insert_into_bst(node: Rc<RefCell<TreeNode>>, val: i32) {
    let mut n = node.borrow_mut();
    if n.val < val {
        check_child(&mut n.right, val);
    } else {
        check_child(&mut n.left, val);
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

    fn test<const M: usize, const N: usize>(
        root: [Option<i32>; M],
        val: i32,
        res: [Option<i32>; N],
    ) {
        assert_eq!(
            insert_into_bst(array_to_tree(root), val),
            array_to_tree(res)
        );
    }

    #[test]
    fn case1() {
        test(
            [Some(4), Some(2), Some(7), Some(1), Some(3)],
            5,
            [Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)],
        );
    }

    #[test]
    fn case2() {
        test(
            [
                Some(40),
                Some(20),
                Some(60),
                Some(10),
                Some(30),
                Some(50),
                Some(70),
            ],
            25,
            [
                Some(40),
                Some(20),
                Some(60),
                Some(10),
                Some(30),
                Some(50),
                Some(70),
                None,
                None,
                Some(25),
            ],
        );
    }
}
