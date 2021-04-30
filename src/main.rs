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
        let (n, m, d) = read!(i64, i64, i64);
        let max = n.max(m);
        let min = n.min(m);
        if max == min {
            println!("YES");
            continue;
        }
        if (max - min - 1) / min + 1 > d {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}
