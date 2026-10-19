#[inline(always)]
fn my_add(a: u32, b: u32) ->u32 {
    a.wrapping_add(b)
}

fn main() {
    let a = 200u32 ;
    let b = 255u32 ;
    println!("my_add({}, {}) = {}", a, b, my_add(a, b)); // Out: my_add(200, 255) = 455
    println!("my_add({}, {}) = {}", u32::MAX, b, my_add(u32::MAX, b)) ; // Out: my_add(4294967295, 255) = 254
}
