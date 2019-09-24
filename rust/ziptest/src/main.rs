struct Test {
    i32s: [i32; 3],
    u64s: [u64; 3],
}

impl Test {
    pub fn get_zip(&self) -> std::iter::Zip<i32, u64> {
        self.i32s.iter().zip(self.u64s.iter())
    }
}

fn main() {
    let i32s: [i32; 3] = [-1, -2, -3];
    let u64s: [u64; 3] = [1, 2, 3];
    let test_value = Test { i32s, u64s, };
    for (s, u) in test_value.get_zip() {
    // for (s, u) in test_value.i32s.iter().zip(test_value.u64s.iter()) {
        println!("{} {}", s, u);
    }
    println!("Hello, world!");
}
