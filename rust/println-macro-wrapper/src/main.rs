// macro_rules! sw_log {
//     ($log_file:ident, $fmt:expr, )) => {
//         println!("Logging to... {}", stringify!($log_file));
//         println!($fmt, $args);
//     };
// }
use std::io::Write as IoWrite;

macro_rules! sw_println {
    ($dst:expr, $fmt:expr) => {
        println!($fmt);
        writeln!($dst, $fmt).expect("sw_println failed!");
    };
    ($dst:expr, $fmt:expr, $($arg:expr),* ) => {
        println!($fmt, $($arg),*);
        writeln!($dst, $fmt, $($arg),*).expect("sw_println failed!");
    };
}

struct FooStruct {
    v: u8,
}

fn main() {
    let mut log_file = Box::new(std::io::sink());
    let x = FooStruct { v: 1 };
    sw_println!(log_file, "cool cats {}", x.v);
    sw_println!(log_file, "cool cats {} {}", x.v, x.v);
    sw_println!(log_file, "cool cats");
}
