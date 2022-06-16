use super::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }

    let mut vec = Vec::new();
    _level_order(root, 0, &mut vec);

    println!("{:?}", vec);

    for level in vec {
        let len = level.len() / 2;
        if !level.iter().take(len).eq(level.iter().rev().take(len)) {
            return false;
        }
    }

    true
}

fn _level_order(
    node: Option<Rc<RefCell<TreeNode>>>,
    level: usize,
    vec: &mut Vec<Vec<Option<i32>>>,
) {
    if vec.len() == level {
        vec.push(Vec::new());
    }

    match node {
        Some(n) => {
            vec[level].push(Some(n.borrow().val));
            _level_order(n.borrow_mut().left.take(), level + 1, vec);
            _level_order(n.borrow_mut().right.take(), level + 1, vec);
        }
        None => {
            vec[level].push(None);
        }
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

    fn test<const N: usize>(root: [Option<i32>; N], res: bool) {
        assert_eq!(is_symmetric(array_to_tree(root)), res);
    }

    #[test]
    fn case1() {
        test(
            [
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(4),
                Some(4),
                Some(3),
            ],
            true,
        );
    }

    #[test]
    fn case2() {
        test(
            [Some(1), Some(2), Some(2), None, Some(3), None, Some(3)],
            false,
        );
    }
}
