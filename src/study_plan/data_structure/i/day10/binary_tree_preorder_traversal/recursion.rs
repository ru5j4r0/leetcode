use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(val) => {
            let mut node = val.borrow_mut();
            let mut vec = vec![node.val];
            vec.append(&mut preorder_traversal(node.left.take()));
            vec.append(&mut preorder_traversal(node.right.take()));
            vec
        }
        None => vec![],
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

    fn test<const M: usize, const N: usize>(root: [Option<i32>; M], res: [i32; N]) {
        assert_eq!(preorder_traversal(array_to_tree(root)), res);
    }

    #[test]
    fn case1() {
        test([Some(1), None, Some(2), Some(3)], [1, 2, 3]);
    }

    #[test]
    fn case2() {
        test([], []);
    }

    #[test]
    fn case3() {
        test([Some(1)], [1]);
    }
}
