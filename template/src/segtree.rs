use std::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Mul},
    usize,
};
#[derive(Default)]
struct STreeNode<T: Default + AddAssign + Add<Output = T> + Copy + Mul<usize, Output = T> + Debug> {
    l: Option<Box<STreeNode<T>>>,
    r: Option<Box<STreeNode<T>>>,
    sum: T,
    fac: T,
}

impl<T: Default + AddAssign + Add<Output = T> + Copy + Mul<usize, Output = T> + Debug>
    STreeNode<T>
{
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

    fn add(&mut self, lpos: usize, rpos: usize, val: T, l: usize, r: usize) {
        if lpos <= l && r <= rpos {
            self.fac += val;
            self.sum += val * (r - l + 1);
            return;
        }
        let mid = (l + r) / 2;
        if lpos <= mid {
            self.l.as_deref_mut().unwrap().add(lpos, rpos, val, l, mid);
        }
        if rpos > mid {
            self.r
                .as_deref_mut()
                .unwrap()
                .add(lpos, rpos, val, mid + 1, r);
        }
        self.sum =
            self.l.as_ref().unwrap().sum + self.r.as_ref().unwrap().sum + self.fac * (r - l + 1);
    }

    fn get(&mut self, lpos: usize, rpos: usize, l: usize, r: usize) -> T {
        if lpos <= l && r <= rpos {
            return self.sum;
        }
        let mid = (l + r) / 2;
        let mut sum = T::default();
        self.l.as_deref_mut().unwrap().fac += self.fac;
        self.l.as_deref_mut().unwrap().sum += self.fac * (mid - l + 1);
        self.r.as_deref_mut().unwrap().fac += self.fac;
        self.r.as_deref_mut().unwrap().sum += self.fac * (r - mid);
        self.fac = T::default();
        if lpos <= mid {
            sum += self.l.as_deref_mut().unwrap().get(lpos, rpos, l, mid);
        }
        if rpos > mid {
            sum += self.r.as_deref_mut().unwrap().get(lpos, rpos, mid + 1, r);
        }
        sum
    }
}

impl<T: Default + AddAssign + Add<Output = T> + Copy + Mul<usize, Output = T> + Debug> Debug
    for STreeNode<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.sum).field(&self.fac).finish()
    }
}
pub struct STree<T: Default + AddAssign + Add<Output = T> + Copy + Mul<usize, Output = T> + Debug> {
    root: Option<Box<STreeNode<T>>>,
    l: usize,
    r: usize,
}

impl<T: Default + AddAssign + Add<Output = T> + Copy + Mul<usize, Output = T> + Debug> STree<T> {
    pub fn new(l: usize, r: usize) -> Self {
        let mut root = Box::new(STreeNode::new());
        root.build(l, r);
        STree {
            root: Some(root),
            l,
            r,
        }
    }
    pub fn add(&mut self, lpos: usize, rpos: usize, val: T) -> Result<(), ()> {
        if lpos < self.l || lpos > self.r || rpos < self.l || rpos > self.r {
            return Err(());
        }
        self.root
            .as_deref_mut()
            .unwrap()
            .add(lpos, rpos, val, self.l, self.r);
        Ok(())
    }
    pub fn get(&mut self, lpos: usize, rpos: usize) -> Result<T, ()> {
        if lpos < self.l || lpos > self.r || rpos < self.l || rpos > self.r {
            return Err(());
        }
        return Ok(self
            .root
            .as_deref_mut()
            .unwrap()
            .get(lpos, rpos, self.l, self.r));
    }
}

#[test]
fn test1() -> Result<(), ()> {
    let l = 1;
    let r = 100000;
    let mut stree = STree::new(l, r);
    for i in l..r + 1 {
        stree.add(i, i, 1)?;
    }
    assert_eq!(r - l + 1, stree.get(l, r).unwrap());
    Ok(())
}

#[test]
fn test2() -> Result<(), ()> {
    let l = 1;
    let r = 100000;
    let mut stree = STree::new(l, r);
    for i in l..r + 1 - 10 {
        stree.add(i, i + 10, 1)?;
    }
    assert_eq!((r - l + 1 - 10) * 11, stree.get(l, r).unwrap());
    Ok(())
}
