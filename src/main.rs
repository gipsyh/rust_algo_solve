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
use std::collections::BTreeSet;
fn main() {
    let _ = read!(usize);
    let mut a = read_vec!(i32);
    let mut set = BTreeSet::new();
    set.insert(0);
    for x in &a {
        let mut tmp = BTreeSet::new();
        for y in set.iter() {
            tmp.insert(y + x);
        }
        for y in tmp.iter() {
            set.insert(*y);
        }
    }
    let mut sum = 0;
    for x in &a {
        sum += x;
    }
    if sum % 2 == 0 && set.contains(&(sum / 2)) {
        println!("1");
        loop {
            match a.iter().position(|a| a % 2 != 0) {
                Some(ans) => {
                    println!("{}", ans + 1);
                    break;
                }
                None => {
                    for i in 0..a.len() {
                        a[i] /= 2;
                    }
                }
            }
        }
    } else {
        println!("0");
    }
}
