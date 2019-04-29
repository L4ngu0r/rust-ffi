extern crate libc;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
    fn get_message() -> libc::c_char;
}

fn main() {
    let input = 4;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
    println!("{}", unsafe { get_message() });
}

#[test]
fn check_double_input() {
    assert_eq!(8, unsafe { double_input(4) });
}
