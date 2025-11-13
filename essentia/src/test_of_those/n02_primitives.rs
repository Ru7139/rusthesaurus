#[test]
fn scalar_types() {
    let the_unit_type = ();
    println!("the_unit_type: {:?}", the_unit_type);

    let bool_type = (true, false);
    println!("bool_type: {:?}", bool_type);

    let signed_intgers = (i8::MAX, i16::MAX, i32::MAX, i64::MAX, i128::MAX, isize::MAX);
    println!("signed_intgers: {:?}", signed_intgers);

    let unsigned_intgers = (u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX, usize::MAX);
    println!("unsigned_intgers: {:?}", unsigned_intgers);

    let floating_point = (f32::MAX, f64::MAX);
    println!("floating_point: {:?}", floating_point);

    let char_type = ("A", "b", "u", "1", char::MAX, char::MIN);
    println!("char_type: {:?}", char_type);
}

#[test]
fn compound_types() {
    let array = [1, 2, 3, 4, 5, 6, 7];
    println!("array: {:?}", array);

    let tuple = (8, 9, 0);
    println!("tuple: {:?}", tuple);
}

#[test]
fn operators() {
    let (a, b) = (10u32, 20u32);

    let c1 = a + b;
    //let c2 = a - b; // attempt to compute `10_u32 - 20_u32`, which would overflow
    let c3 = b - a;
    assert_eq!(c1, 30);
    assert_eq!(c3, 10);

    let d = a * b;
    assert_eq!(d, 200);

    let e1 = a / b;
    let e2 = b / a;
    assert_eq!(e1, 0);
    assert_eq!(e2, 2);

    let f = a % b;
    assert_eq!(f, 10);

    let g = a << 1;
    assert_eq!(g, 20);

    let (t, f) = (true, false); // 1，0
    let j0 = t & f; // bitwise AND  // false
    let j1 = t && f; // AND         // false
    let j2 = t | f; // bitwise OR   // true
    let j3 = t || f; // OR          // true
    let j4 = t ^ f; // bitwise XOR  // true
    let j5 = f & t; //              // false
    println!("j0: {j0}\nj1\n{j1}\nj2\n{j2}\nj3: {j3}\nj4: {j4}\nj5: {j5}");
}

#[test]
fn array_slice() {
    let array: [u32; 101] = std::array::from_fn(|x| x as u32);

    let slice0 = &array[0..10]; // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    println!("{:?}", slice0);

    let slice1 = &array[10..=20]; // [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
    println!("{:?}", slice1);
}
