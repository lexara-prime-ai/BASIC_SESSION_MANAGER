#![allow(unused)]
#![allow(non_snake_case)]

#[macro_export]
macro_rules! logger {
    ($val: expr) => {
        println!("{:#?}", $val);
    };
}
