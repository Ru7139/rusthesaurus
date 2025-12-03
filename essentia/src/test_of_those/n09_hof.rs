#[test]
fn long_hof() {
    let _vec = (0..100)
        .into_iter()
        .enumerate()
        .map(|(idx, value)| (idx, value * 2))
        .filter(|(_, value_double)| value_double % 3 == 0)
        .filter_map(|(i, v)| {
            if v > 20 {
                Some((i, v))
            } else {
                Some((i, v * 100))
            }
        })
        .flat_map(|(i, v)| [i as i32, v])
        .collect::<Vec<i32>>();
}
