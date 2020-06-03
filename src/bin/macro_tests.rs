macro_rules! vv {
    ( $($x: expr),* ) => {
        {
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*

            temp_vec
        }
    };
}
#[macro_export]
macro_rules! vc {
    ( $( $x:expr ),* ) => {
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
    let v = vv!(1, 2, 3);
    println!{"{:?}", v};
}
