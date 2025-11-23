#[allow(unused)]
mod widgets;

fn main() {
    let acc =
        widgets::p01_lines_counter::count_all_the_rs_files_and_lines_under_the_rusthesaurus("")
            .unwrap();
    println!("files = {}, lines = {}", acc.0, acc.1);
}
