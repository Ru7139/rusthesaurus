// #[rustfmt::skip]
// fn random_vec_generate<T>(length: usize) -> Option<Vec<T>>
// where rand::distr::StandardUniform: rand::distr::Distribution<T>,
// {   if length == 0 { return None };
//     let vec = (1..=length).into_iter().map(|_| rand::random::<T>()).collect::<Vec<T>>();
//     Some(vec)
// }

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[test]
fn get_some_random_numbers_standard_uniform_distribution() {
    // single random
    println!("x ---> {}", rand::random::<u64>());

    // iter random
    #[rustfmt::skip]
    println!("u32 ---> {:?}", rand::random_iter().take(4).collect::<Vec<u32>>());
    #[rustfmt::skip]
    println!("f32 ---> {:?}", rand::random_iter().take(4).collect::<Vec<f32>>()); // 0~1.0
    #[rustfmt::skip]
    println!("f64 ---> {:?}", rand::random_iter().take(4).collect::<Vec<f64>>()); // 0~1.0

    // range
    println!("0.0 ~ 100.0 ---> {}", rand::random_range(0.0..100.0));

    // bool with ratio
    println!("ratio --> {}", rand::random_ratio(3, 5)); // chance = 3/5 = 60%

    // bool with p
    let p = 0.3;
    let p_p = p * 100.0;
    println!("{}% chance to get true ---> {}", p_p, rand::random_bool(p));
}

#[test]
fn get_normal_gaussian_distribute() {
    use rand_distr::Distribution;
    let vec_len = 1_000_000;
    // fn new(mean: F, std_dev: F) // mean决定中心位置，stddev为标准差
    let normal = rand_distr::Normal::new(0.0, 1.0).unwrap();

    let timer = std::time::Instant::now(); // 10m single ---> rayon ---> 1m rayon
    let normal_dis_vec = (1..=vec_len)
        .into_par_iter()
        .map_init(|| rand::rng(), |rng, _| normal.sample(rng))
        .collect::<Vec<f64>>();
    dbg!(timer.elapsed()); // 10m ---> 5.1502495s ---> 663.021708ms  ---> 91.352834ms

    let mean_value = normal_dis_vec.into_par_iter().sum::<f64>() / vec_len as f64;
    dbg!(timer.elapsed()); // 10m ---> 54.34ms ---> 22.181ms ---> 1.54921ms

    dbg!((mean_value - 0.0).abs());
}
