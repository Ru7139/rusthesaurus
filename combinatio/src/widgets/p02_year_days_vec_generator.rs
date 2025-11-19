const NORMAL_YEAR: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_YEAR: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const YEAR_DAYS: [[i32; 12]; 2] = [NORMAL_YEAR, LEAP_YEAR];

const SAKAMOTO_WEEKDAY_ARRAY: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

#[allow(unused)]
pub fn all_year_days_yyyymmdd_vec_gen(year: u32) -> Vec<(u32, u32)> {
    let is_leap_year =
        |y: u32| -> usize { (y % 4 == 0 && (y % 400 == 0 || y % 100 != 0)) as usize };

    YEAR_DAYS[is_leap_year(year)]
        .into_iter()
        .enumerate()
        .flat_map(|(m, d)| (1..=d).map(move |day| year * 10000 + (m as u32 + 1) * 100 + day as u32))
        .map(|ymd| {
            let (y, m, d) = (ymd / 10000, (ymd / 100) % 100, ymd % 100);
            let z = if m >= 3 { y } else { y - 1 };
            let part = z + z / 4 - z / 100 + z / 400;

            let week_day_number = (part + SAKAMOTO_WEEKDAY_ARRAY[(m - 1) as usize] as u32 + d) % 7;
            (ymd, week_day_number)
        })
        .collect::<Vec<(u32, u32)>>()
}

#[rustfmt::skip]
#[cfg(test)]
#[test]
fn test_of_aydymdvg() {
    let vec = all_year_days_yyyymmdd_vec_gen(2025);

    for (ymd, day_number) in &vec { println!("{} is the day {} of that week", ymd, day_number); } // day 0 means sunday

    vec.into_iter()
        .filter_map(|x| {
            if (x.1 == 0) || (x.1 == 6) { None }
                else { Some(x) }
        })
        .for_each(|(ymd, day_number)| println!("{} is the day {} of that week", ymd, day_number));
}
