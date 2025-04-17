#[allow(unused_variables, unused_assignments, dead_code)]
fn main() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255)
}

fn _type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
