use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[allow(unused)]
pub fn count_all_the_rs_files_and_lines_under_the_rusthesaurus(
    path: &str,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let timer = std::time::Instant::now();

    let crate_root = std::path::Path::new(path);
    let crate_toml = crate_root.join("Cargo.toml");
    let toml_string = std::fs::read_to_string(crate_toml)?;

    let mems = toml_string.find("members").unwrap();
    let arrs = toml_string[mems..].find("[").unwrap() + mems;
    let arre = toml_string[arrs..].find("]").unwrap() + arrs;

    let members_slice = &toml_string[arrs + 1..arre];
    let members_pathbuf_vec = members_slice
        .split(",")
        .map(|x| x.trim().trim_matches('"').to_string())
        .filter(|x| !x.is_empty())
        .map(|member_name| (member_name.clone(), crate_root.join(member_name)))
        .collect::<Vec<(String, std::path::PathBuf)>>();

    let (total_files, total_lines): (usize, usize) = members_pathbuf_vec
        .par_iter()
        .map(|(member_name, path)| {
            let (files, lines) = walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| entry.file_type().is_file())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
                .map(|entry| {
                    let file_path = entry.path();
                    let lines_count = std::fs::read_to_string(file_path)
                        .map(|content| content.lines().count())
                        .unwrap_or(0);
                    (1usize, lines_count)
                })
                .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
            (member_name.clone(), (files, lines))
        })
        .map(|(member_name, (files, lines))| {
            println!(
                "{:>15} ---> {:>4} files, {:>5} lines",
                member_name, files, lines
            );
            (files, lines)
        })
        .reduce(
            || (0usize, 0usize),
            |a: (usize, usize), b: (usize, usize)| (a.0 + b.0, a.1 + b.1),
        );

    println!("RsFileCountingTimer: {:?}", timer.elapsed());

    Ok((total_files, total_lines))
}

#[cfg(test)]
#[test]
fn test_of_catrfalutr() {
    let acc = count_all_the_rs_files_and_lines_under_the_rusthesaurus("../").unwrap();
    println!("rs files: {}", acc.0);
    println!("rs codes lines: {}", acc.1);
}
