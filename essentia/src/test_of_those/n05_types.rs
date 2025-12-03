#[test]
fn casting() {
    let number_a = 65.4321f64;
    let integer_u8: u8 = number_a as u8;
    let character: char = integer_u8 as char;
    dbg!(integer_u8, character);

    unsafe {
        dbg!(300.1_f64.to_int_unchecked::<u8>());
        dbg!((-100.2_f64).to_int_unchecked::<u8>());
        dbg!(f32::NAN.to_int_unchecked::<u8>());
    }
}

#[test]
fn literals() {}

#[test]
fn inference() {}

#[test]
fn aliasing() {}
