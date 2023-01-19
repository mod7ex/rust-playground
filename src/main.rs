#![allow(unused)]

// https://www.youtube.com/watch?v=VGk95NXaafs&ab_channel=TensorProgramming

macro_rules! x_and_y {
    (x => $e:expr) => ( println!("x: {}", $e) );
    (y => $e:expr) => ( println!("y: {}", $e) );
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("you called a function with identifier {:?}", stringify!($func_name));
        };
    }
}

macro_rules! print_ex {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}

macro_rules! exam {
    ($a:expr; and $b:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($a),
            stringify!($b),
            $a && $b
        );
    };

    ($a:expr; or $b:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($a),
            stringify!($b),
            $a || $b
        );
    };
}

macro_rules! comprehend {
    ($id1: ident | $id2: ident <- [$start: expr; $end: expr], $cond: expr) => {
        {
            let mut vec = Vec::new();

            for num in $start..$end + 1 {
                if $cond(num) {
                    vec.push(num);
                }
            }

            vec
        }
    };
}

fn try_comprehend_macro(){
    fn even(x: i32) -> bool {
        x%2 == 0
    }

    fn odd(x: i32) -> bool {
        x%2 != 0
    }

    let evens = comprehend!(x | x <- [1; 10], even);
    let odds = comprehend!(x | x <- [1; 10], odd);
    println!("{:?}", evens);
    println!("{:?}", odds);
}

use std::collections::HashMap;

macro_rules! new_map {
    ($($key:expr => $val:expr)*) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

macro_rules! new_map_with_comma {
    ($($key:expr => $val:expr),+) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

macro_rules! calc {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    (eval $e:expr, $(eval $es:expr),+) => {
        {
            calc! { eval $e }
            calc! { $(eval $es),+ }
        }
    };
}

fn main() {
    calc! { eval 4 * 3 }
    calc! {
        eval 4 * 3,
        eval 3 * 10,
        eval 3 * 10,
        eval 2 * 5,
        eval 4 * 6
    }

    /*
    // you can call it using [], {} or ()
    let map = new_map![
        "one" => 1
        "tow" => 2
        "three" => 3
    ];
        
    let _map = new_map_with_comma!{
        "one" => 1,
        "tow" => 2,
        "three" => 3
    };
    
    println!("{:#?}", map);
    */

    // try_comprehend_macro();

    // x_and_y!(x => 63);

    /*
    build_fn!(some_thing);
    some_thing();
    */

    /*
    print_ex!(1 + 3);
    print_ex!({
        let x = 2;
        let y = 3;
        x + y + 1
    });
    */

    /*
    exam!(false; and false);
    exam!(true; and false);
    exam!(false; or true);
    */
}



