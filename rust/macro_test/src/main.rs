struct Pos(i32, i32);

macro_rules! my_vec {
    ( $(($x:expr,$y:expr)),* $(,)?) => {
        [
            $(
                Pos($y,$x),
            )*
        ]
    };

    /*
    ( ($x:expr,$y:expr) ) => {
        $(
            Pos($y,$x)
        )*
    };
    */
}

fn main() {
    println!("Hello, world!");
    let x = my_vec![
        (1,2),
        (2,3),
        (3,4),
        (3,4),
    ];
    for v in x.iter() {
        println!("{:?}", (v.0,v.1));
    }

    /*
    let y = my_vec!(3,5);
    println!("{:?}", (y.0,y.1));
    */
}
