use crate::utils::read_lines;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum NodeType {
    File,
    Dir(String),
}

struct TreeNode {
    value: NodeType,
    size: Option<usize>,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(
        value: NodeType,
        size: Option<usize>,
        parent: Option<Weak<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            size,
            children: vec![],
            parent,
        }))
    }

    fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    fn get_dir(&self, name: &str) -> Option<Rc<RefCell<TreeNode>>> {
        for child in &self.children {
            if let NodeType::Dir(cname) = &child.borrow_mut().value {
                if cname == name {
                    return Some(Rc::clone(&child));
                }
            }
        }

        return None;
    }

    fn size(&mut self) -> usize {
        if let Some(size) = self.size {
            return size;
        }
        let mut size = 0;
        for child in &self.children {
            size += child.borrow_mut().size();
        }
        self.size = Some(size);

        return size;
    }

    fn bfs(
        node: Rc<RefCell<TreeNode>>,
        queue: &mut VecDeque<Rc<RefCell<TreeNode>>>,
        res: &mut Vec<Rc<RefCell<TreeNode>>>,
    ) {
        for child in &node.borrow().children {
            queue.push_back(Rc::clone(child));
        }
        res.push(Rc::clone(&node));

        if let Some(curr) = queue.pop_front() {
            TreeNode::bfs(curr, queue, res);
        }
    }
}

pub fn run() -> String {
    let mut lines = read_lines("7").unwrap();
    let root_dir = TreeNode::new(NodeType::Dir("/".to_string()), None, None);
    let mut current_dir = Rc::clone(&root_dir);
    lines.next();

    for line in lines {
        let line = line.unwrap();
        let mut line = line.split_whitespace();

        match line.next() {
            Some("$") => match line.next() {
                Some("cd") => match line.next() {
                    Some("..") => {
                        let parent =
                            Weak::upgrade(&current_dir.borrow_mut().parent.as_ref().unwrap())
                                .unwrap();
                        current_dir = Rc::clone(&parent);
                    }
                    Some(s) => {
                        let child_dir = current_dir.borrow_mut().get_dir(s).unwrap();

                        current_dir = child_dir;
                    }
                    _ => (),
                },
                Some("ls") => {}
                _ => (),
            },
            Some("dir") => {
                let child = TreeNode::new(
                    NodeType::Dir(line.next().unwrap().to_string()),
                    None,
                    Some(Rc::downgrade(&current_dir)),
                );
                current_dir.borrow_mut().add_child(Rc::clone(&child));
            }
            Some(file_size) => {
                let child = TreeNode::new(
                    NodeType::File,
                    Some(file_size.parse::<usize>().unwrap()),
                    Some(Rc::downgrade(&current_dir)),
                );
                current_dir.borrow_mut().add_child(Rc::clone(&child));
            }
            _ => (),
        }
    }
    let mut queue = VecDeque::new();
    let mut res = vec![];
    TreeNode::bfs(root_dir, &mut queue, &mut res);
    res.iter()
        .filter(|x| {
            let size = x.borrow_mut().size();
            if let NodeType::Dir(_) = x.borrow().value {
                if size <= 100000 {
                    return true;
                }
            }
            return false;
        })
        .fold(0, |acc, x| {
            return acc + x.borrow_mut().size();
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(1582412.to_string(), run());
    }
}
