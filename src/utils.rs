use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use array2d::Array2D;

/// Reads the corresponding `input_name` file for this day from the `inputs` directory.
///
/// `self_name` should be a call to the `file!()` macro from the calling file.
///
/// # Panics
///
/// Panics if the file in `file_path` has no name or is not nested two directories down from
/// the directory containing the `inputs` folder.
pub fn read_input_file(self_name: &str, input_name: &str) -> String {
    let current_day: &str = Path::new(self_name)
        .file_stem()
        .and_then(|x| x.to_str())
        .expect("Calling file should have a name of form dayXX.rs");

    let input_dir: PathBuf = Path::new(self_name)
        .ancestors()
        .nth(3)
        .expect("Calling file should be located 3 directories down")
        .join("inputs")
        .join(current_day);
    let input_filename = input_dir.join(input_name);
    fs::read_to_string(input_filename).expect("Opening input file failed")
}

pub fn read_2d_map(contents: String) -> Array2D<char> {
    let lines = contents.split('\n').take_while(|x| !x.is_empty());
    let height = contents.matches('\n').count();
    let width = contents.find('\n').unwrap();
    Array2D::from_iter_row_major(lines.flat_map(|x| x.chars()), height, width).unwrap()
}

pub fn read_2d_map_to_u8(contents: String) -> Array2D<u8> {
    let lines = contents.split('\n').take_while(|x| !x.is_empty());
    let height = contents.matches('\n').count();
    let width = contents.find('\n').unwrap();
    Array2D::from_iter_row_major(
        lines.flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as u8)),
        height,
        width,
    )
    .unwrap()
}
pub fn pretty_print<T: Display>(map: &Array2D<T>) {
    for row in map.rows_iter() {
        for item in row {
            print!("{}", item)
        }
        println!()
    }
}
