use user_a_sys_test_haobo::echo;
use user_a_sys_test_haobo::add as add2;
use user_b_sys_test_haobo::add;
fn main() {
    let n = 0;
    unsafe {
        println!("add: {}", add(n))
    }
    unsafe {
        println!("add2: {}", add2(n))
    }
    unsafe {
        println!("echo: {}", echo(n))
    }
}

