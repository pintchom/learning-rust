fn main() {
    let (x, y): (i32, i32); // THESE CANNOT BE DIFFERENT TYPES

    (x, ..) = (3, ..);
    [.., y] = [1, 2];
    // x = 3;
    // y = 2;

    assert_eq!([x, y], [3, 2]);
    println!("Success");
}
