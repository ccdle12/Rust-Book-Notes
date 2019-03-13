fn main() {
    // We reference and borrow the i32 from x and assign to y.
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // The reason this fails is because we need to deference the y, to point the value it is
    // refernencing.
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    // Here we are using Box instead of a reference to x.
    let x = 10;
    let y = Box::new(x);
    assert_eq!(10, x);
    assert_eq!(10, *y);
}
