use std::{
    borrow::{Borrow, BorrowMut},
    usize,
};

struct STreeNode {
    l: Option<Box<STreeNode>>,
    r: Option<Box<STreeNode>>,
    sum: usize,
}

impl STreeNode {
    pub fn new() -> Self {
        STreeNode {
            l: None,
            r: None,
            sum: 0,
        }
    }
    pub fn build(&mut self, l: usize, r: usize) {
        if l == r {
            println!("{}", l);
            return;
        }
        let mut ln = Box::new(STreeNode::new());
        let mut rn = Box::new(STreeNode::new());
        let mid = (l + r) / 2;
        ln.build(l, mid);
        rn.build(mid + 1, r);
        self.l = Some(ln);
        self.r = Some(rn);
    }
    pub fn add(&mut self, pos: usize, val: usize, l: usize, r: usize) {
        if l == r {
            self.sum += val;
            return;
        }
        let mid = (l + r) / 2;
        if pos <= mid {
            let tmp = self.l.unwrap().as_mut();
            .add(pos, val, l, mid);
        }
    }
}
struct STree {
    root: Option<Box<STreeNode>>,
    l: usize,
    r: usize,
}

impl STree {
    pub fn new(l: usize, r: usize) -> Self {
        let mut root = Box::new(STreeNode::new());
        root.build(l, r);
        STree {
            root: Some(root),
            l,
            r,
        }
    }
    pub fn add(&self, p: usize, val: usize) {}
}

#[test]
fn test1() {
    let stree = STree::new(1, 100000);
}
