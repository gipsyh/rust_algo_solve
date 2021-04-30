use std::io;

#[allow(dead_code)]
fn gets() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($ty:ty) => {{
        let line = gets();
        let mut it = line.split_whitespace();
        let mut v = Vec::new();
        while let Some(value) = it.next() {
            v.push(value.parse::<$ty>().unwrap());
        }
        v
    }};
}

#[allow(unused_macros)]
macro_rules! read {
    ( $ty:ty) => {
        gets().split_whitespace().next().unwrap().parse::<$ty>().unwrap()
    };
    ( $($ty:ty),* ) => {{
        let line = gets();
        let mut it = line.split_whitespace();
        ( $(it.next().unwrap().parse::<$ty>().unwrap(),)* )
    }};
    ( $( $ty:ty ),+ ,) => {
		read![ $( $ty ),* ]
    };
}
fn main() {
    for _ in 0..read!(i64) {
        let n = read!(usize);
        let u = read_vec!(usize);
        let s = read_vec!(i64);
        let mut idx: Vec<usize> = (0..n).collect();
        let mut v: Vec<Vec<usize>> = vec![vec![]; n];
        let mut ans = vec![0; n];
        idx.sort_by_key(|a| s[*a] * -1);
        for x in &idx {
            v[u[*x] - 1].push(*x);
        }
        for v in v {
            let mut tmp = 0;
            let mut pre: Vec<i64> = vec![];
            for x in &v {
                tmp += s[*x];
                pre.push(tmp);
            }
            for i in 0..v.len() {
                ans[i] += pre[v.len() / (i + 1) * (i + 1) - 1];
            }
        }
        for x in ans {
            print!("{} ", x);
        }
        println!();
    }
}
