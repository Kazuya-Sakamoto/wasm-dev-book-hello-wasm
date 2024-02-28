#[no_mangle]
// コンパイラにこの関数の名前を変更しないよう指示
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// index.html > imports.env.date_now を参照できるようにする
extern {
    fn date_now() -> f64;
}

#[no_mangle]
pub fn get_timestamp() -> f64 {
    unsafe {
        date_now()
    }
}
