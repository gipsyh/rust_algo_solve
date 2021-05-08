use std::{
    ops::{Add, AddAssign},
    usize,
};
#[derive(Default)]
struct STreeNode<T: Default + AddAssign + Add<Output = T> + Copy> {
    l: Option<Box<STreeNode<T>>>,
    r: Option<Box<STreeNode<T>>>,
    sum: T,
}

impl<T: Default + AddAssign + Add<Output = T> + Copy> STreeNode<T> {
    // type Output = T;
    fn new() -> Self {
        STreeNode::default()
    }
    fn build(&mut self, l: usize, r: usize) {
        if l == r {
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
    fn add(&mut self, pos: usize, val: T, l: usize, r: usize) {
        if l == r {
            self.sum += val;
            return;
        }
        let mid = (l + r) / 2;
        if pos <= mid {
            self.l.as_deref_mut().unwrap().add(pos, val, l, mid);
        } else {
            self.r.as_deref_mut().unwrap().add(pos, val, mid + 1, r);
        }
        self.sum = self.l.as_ref().unwrap().sum + self.r.as_ref().unwrap().sum;
    }

    fn get(&self, lpos: usize, rpos: usize, l: usize, r: usize) -> T {
        if lpos <= l && r <= rpos {
            return self.sum;
        }
        let mid = (l + r) / 2;
        let mut sum = T::default();
        if lpos <= mid {
            sum += self.l.as_ref().unwrap().get(lpos, rpos, l, mid);
        }
        if rpos > mid {
            sum += self.r.as_ref().unwrap().get(lpos, rpos, mid + 1, r);
        }
        sum
    }
}

pub struct STree<T: Default + AddAssign + Add<Output = T> + Copy> {
    root: Option<Box<STreeNode<T>>>,
    l: usize,
    r: usize,
}

impl<T: Default + AddAssign + Add<Output = T> + Copy> STree<T> {
    pub fn new(l: usize, r: usize) -> Self {
        let mut root = Box::new(STreeNode::new());
        root.build(l, r);
        STree {
            root: Some(root),
            l,
            r,
        }
    }
    pub fn add(&mut self, pos: usize, val: T) -> Result<(), ()> {
        if pos < self.l || pos > self.r {
            return Err(());
        }
        self.root
            .as_deref_mut()
            .unwrap()
            .add(pos, val, self.l, self.r);
        Ok(())
    }
    pub fn get(&self, lpos: usize, rpos: usize) -> Result<T, ()> {
        if lpos < self.l || lpos > self.r || rpos < self.l || rpos > self.r {
            return Err(());
        }
        return Ok(self.root.as_ref().unwrap().get(lpos, rpos, self.l, self.r));
    }
}

#[test]
fn test1() -> Result<(), ()> {
    let l = 1;
    let r = 100;
    let mut stree = STree::new(1, 100);
    for i in l..r + 1 {
        stree.add(i, 1)?;
    }
    assert_eq!(r - l + 1, stree.get(l, r).unwrap());
    Ok(())
}
