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
use ::std::cmp::min;
fn main() {
    for _ in 0..read!(usize) {
        let (_, mut k) = read!(usize, usize);
        let mut add = 0;
        let mut a = read_vec!(usize);
        for i in 0..a.len() {
            let tmp = min(k, a[i]);
            a[i] -= tmp;
            k -= tmp;
            add += tmp;
        }
        for i in 0..a.len() {
            if i != a.len() - 1 {
                print!("{} ", a[i]);
            } else {
                println!("{}", a[i] + add);
            }
        }
    }
}
