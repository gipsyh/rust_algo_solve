use std::collections::BTreeMap;
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
    for _ in 0..read!(usize) {
        let n = read!(usize);
        let v = read_vec!(i64);
        let mut a: BTreeMap<i64, i64> = BTreeMap::new();
        let mut ans = 0;
        let mut last = 0;
        for i in 0..n {
            last += *a.entry(v[i]).or_insert(0);
            ans += last;
            *a.entry(v[i]).or_insert(0) += (i + 1) as i64;
        }
        println!("{}", ans);
    }
}
