macro_rules! println_mem_props {
    ($($t:ty),* $(,)?) => {
        $(
            println!("{type_name} has stride {stride} and alignment {alignment}",
                type_name = core::any::type_name::<$t>(),
                stride = core::mem::size_of::<$t>(),
                alignment = core::mem::align_of::<$t>(),
            );
        )*
    };
}

fn main() {
    println_mem_props!((), char, u8, u128, i64, f32, f64, &str, bool);
}
