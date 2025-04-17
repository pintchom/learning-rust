#[allow(unused_variables, unused_assignments)]
fn main() {
    let x = 5;
    let mut y: u32 = 5;

    y = x; // THESE TYPES NEED TO MATCH TO RE-ASSIGN IMMUTABLE

    let _z = 10;

    println!("Success")
}

// default type of x is i32, however if undefined, can inherit type of assignee (y: u32 = x: undefined_type)
