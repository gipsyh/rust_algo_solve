use std::{io, usize};

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
        let _ = read!(usize);
        let a = read_vec!(usize);
        let mut ans: usize = 0;
        let mut pre: Vec<usize> = vec![];
        for x in &a {
            ans = ans ^ x;
            pre.push(ans);
        }
        if ans == 0 {
            println!("YES");
            continue;
        }
        let mut _ans = "NO";
        for i in 0..a.len() - 1 {
            for j in i + 1..a.len() {
                if pre[i] == pre[j] ^ pre[i] && pre[i] == ans ^ pre[j] {
                    _ans = "YES";
                }
            }
        }
        println!("{}", _ans);
    }
}
