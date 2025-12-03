#[test]
fn let_usage() {
    let a = 10;
    let b = a * a;
    let _ = b;
    let c = b * b;
    _ = c;
    let _d = c + 1;
    let mut e = 20;
    e -= -1;
    dbg!(e);
    let e = 30;
    dbg!(e);
}
