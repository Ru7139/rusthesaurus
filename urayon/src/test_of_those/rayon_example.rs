use std::sync::LazyLock;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

static _LAZY_ARRAY: LazyLock<[u32; 1000]> =
    LazyLock::new(|| std::array::from_fn(|x| (x + 1) as u32));

#[test]
fn rayon_speed_test() {
    // let vec = (0..65535) // log2(u32::MAX) = 65535.999 // 手动测试时使用最大值
    let vec = (0..100) // cargo test时使用这个
        .into_iter() // 50k * 1k * 4Bytes = 200MB
        .map(|_| (1..=1000).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let timer_normal = std::time::Instant::now();
    let _ = vec
        .clone()
        .into_iter()
        .flat_map(|x| x.into_iter().map(|y| y * y))
        .collect::<Vec<u32>>();
    dbg!(timer_normal.elapsed()); // 1436.606ms // Debug

    let timer_rayon = std::time::Instant::now();
    let _ = vec
        .clone()
        .into_par_iter()
        .flat_map(|x| x.into_par_iter().map(|y| y * y))
        .collect::<Vec<u32>>();
    dbg!(timer_rayon.elapsed()); // 394.405ms // Debug

    let timer_rayon2 = std::time::Instant::now();
    let _ = vec
        .clone()
        .into_par_iter()
        .flat_map_iter(|x| x.into_iter().map(|x| x * x))
        .collect::<Vec<u32>>();
    dbg!(timer_rayon2.elapsed()); // 260.633ms // Debug

    let timer_rayon3 = std::time::Instant::now();
    let _ = vec
        .into_par_iter()
        .flat_map_iter(|x| x.into_iter().map(|x| x * x))
        .collect::<Vec<u32>>();
    dbg!(timer_rayon3.elapsed()); // 170.762ms // Debug
}
