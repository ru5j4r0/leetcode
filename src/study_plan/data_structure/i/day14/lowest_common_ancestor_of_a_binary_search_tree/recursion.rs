use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(_lowest_common_ancestor(
        root.unwrap(),
        p.unwrap().borrow().val,
        q.unwrap().borrow().val,
    ))
}

fn _lowest_common_ancestor(node: Rc<RefCell<TreeNode>>, p: i32, q: i32) -> Rc<RefCell<TreeNode>> {
    let n = node.borrow();
    let val = n.val;

    if p < val && q < val {
        _lowest_common_ancestor(Rc::clone(n.left.as_ref().unwrap()), p, q)
    } else if val < p && val < q {
        _lowest_common_ancestor(Rc::clone(n.right.as_ref().unwrap()), p, q)
    } else {
        Rc::clone(&node)
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

    fn find(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node.as_ref() {
            let n = n.borrow();
            if n.val == val {
                node.clone()
            } else {
                find(n.left.clone(), val).or(find(n.right.clone(), val))
            }
        } else {
            None
        }
    }

    fn test<const N: usize>(root: [Option<i32>; N], p: i32, q: i32, res: i32) {
        let root = array_to_tree(root);
        assert_eq!(
            lowest_common_ancestor(root.clone(), find(root.clone(), p), find(root.clone(), q)),
            find(root, res)
        );
    }

    #[test]
    fn case1() {
        test(
            [
                Some(6),
                Some(2),
                Some(8),
                Some(0),
                Some(4),
                Some(7),
                Some(9),
                None,
                None,
                Some(3),
                Some(5),
            ],
            2,
            8,
            6,
        );
    }

    #[test]
    fn case2() {
        test(
            [
                Some(6),
                Some(2),
                Some(8),
                Some(0),
                Some(4),
                Some(7),
                Some(9),
                None,
                None,
                Some(3),
                Some(5),
            ],
            2,
            4,
            2,
        );
    }

    #[test]
    fn case3() {
        test([Some(2), Some(1)], 2, 1, 2);
    }
}
