// Purpose: syntax for destructuring tuple as parameter
//
struct A(i32, i32);

fn test(A(a, b): A) -> i32 {
    a + b
}

fn main() {
    let v = A(1, 2);
    println!("Hello, world! {}", test(v));
}
