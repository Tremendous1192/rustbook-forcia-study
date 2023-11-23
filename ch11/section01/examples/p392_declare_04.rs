macro_rules! my_vec {
    ($x:ty)=>
    {
        {
            let temp_vec:Vec<$x>=Vec::new();
            temp_vec
        }
    };

    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let x = my_vec![0];
    println!("{:?}", x);

    let y = my_vec![0, 1, 2];
    println!("{:?}", y);

    let z = my_vec![i32];
    println!("{:?}", z);
}
