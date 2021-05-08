use std::usize;
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
            self.l.as_deref_mut().unwrap().add(pos, val, l, mid);
        } else {
            self.r.as_deref_mut().unwrap().add(pos, val, mid + 1, r);
        }
        self.sum = self.l.as_ref().unwrap().sum + self.r.as_ref().unwrap().sum;
    }

    pub fn get(&self, lpos: usize, rpos: usize, l: usize, r: usize) -> usize {
        if lpos <= l && r <= rpos {
            return self.sum;
        }
        let mid = (l + r) / 2;
        let mut sum = 0;
        if lpos <= mid {
            sum += self.l.as_ref().unwrap().get(lpos, rpos, l, mid);
        }
        if rpos > mid {
            sum += self.r.as_ref().unwrap().get(lpos, rpos, mid + 1, r);
        }
        sum
    }
}

pub struct STree {
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
    pub fn add(&mut self, pos: usize, val: usize) -> Result<(), ()> {
        if pos < self.l || pos > self.r {
            return Err(());
        }
        self.root
            .as_deref_mut()
            .unwrap()
            .add(pos, val, self.l, self.r);
        Ok(())
    }
    pub fn get(&self, lpos: usize, rpos: usize) -> Result<usize, ()> {
        if lpos < self.l || lpos > self.r || rpos < self.l || rpos > self.r {
            return Err(());
        }
        return Ok(self.root.as_ref().unwrap().get(lpos, rpos, self.l, self.r));
    }
}

#[test]
fn test1() -> Result<(), ()> {
    let mut stree = STree::new(1, 100);
    stree.add(1, 55555)?;
    stree.add(2, 44444)?;
    stree.add(3, 1)?;
    println!("{}", stree.get(1, 100).unwrap());
    Ok(())
}
