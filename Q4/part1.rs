/// find one word: XMAS.
///
/// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words.
/// It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them.
///
/// MMMSXXMASM
/// MSAMXMSMSA
/// AMXSXMAAMM
/// MSAMASMSMX
/// XMASAMXAMM
/// XXAMMXXAMA
/// SMSMSASXSS
/// SAXAMASAAA
/// MAMMMXMMMM
/// MXMXAXMASX
///
/// In this word search, XMAS occurs a total of 18 times;
/// here's the same word search again, but where letters not involved in any XMAS have been replaced with .:
///
/// ....XXMAS.
/// .SAMXMS...
/// ...S..A...
/// ..A.A.MS.X
/// XMASAMX.MM
/// X.....XA.A
/// S.S.S.S.SS
/// .A.A.A.A.A
/// ..M.M.M.MM
/// .X.X.XMASX
use std::convert::TryInto;
use std::error::Error;
use std::fs;

enum Direction {
    LeftToRight,
    RightToLeft,
}

fn search_overlapping(search_target: &str, content_to_search: &str) -> i32 {
    if search_target.is_empty() {
        return 0; // avoid infinite loop
    }

    let mut count = 0;
    let mut pos = 0;

    while let Some(found) = content_to_search[pos..].find(search_target) {
        count += 1;
        pos += found + 1; // move one character forward to allow overlap
    }

    count
}

fn collect_diagonal(
    lines: &[&str],
    start_row: i32,
    start_col: i32,
    direction_to_follow: Direction,
) -> String {
    let mut diag = String::new();
    let mut row = start_row;
    let mut col = start_col;

    while (row as usize) < lines.len() {
        if col >= 0 && (col as usize) < lines[row as usize].len() {
            diag.push(lines[row as usize].chars().nth(col as usize).unwrap());
        }

        row += 1;
        match direction_to_follow {
            Direction::LeftToRight => col += 1,
            Direction::RightToLeft => col -= 1,
        }
    }

    diag
}

fn search_horizontal(search_target: &str, content_to_search: &[&str]) -> i32 {
    content_to_search
        .iter()
        .map(|line| search_overlapping(search_target, line))
        .sum()
}

fn search_vertical(search_target: &str, content_to_search: &[&str]) -> i32 {
    let max_len = content_to_search
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    (0..max_len)
        .map(|col| {
            let col_string: String = content_to_search
                .iter()
                .filter_map(|line| line.chars().nth(col))
                .collect();
            search_overlapping(search_target, &col_string)
        })
        .sum()
}

fn search_diagonal(search_target: &str, content_to_search: &[&str]) -> i32 {
    let max_len = content_to_search
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);
    let rows = content_to_search.len();

    let mut count = 0;

    // top left bottom right diag
    for col in 0..max_len {
        let diag = collect_diagonal(
            content_to_search,
            0,
            col.try_into().unwrap(),
            Direction::LeftToRight,
        );
        count += search_overlapping(search_target, &diag);
    }

    for row in 1..rows {
        let diag = collect_diagonal(
            content_to_search,
            row.try_into().unwrap(),
            0,
            Direction::LeftToRight,
        );
        count += search_overlapping(search_target, &diag);
    }

    // Top-right to bottom-left diagonals
    for col in (0..max_len).rev() {
        let diag = collect_diagonal(
            content_to_search,
            0,
            col.try_into().unwrap(),
            Direction::RightToLeft,
        );
        count += search_overlapping(search_target, &diag);
    }

    for row in 1..rows {
        let diag = collect_diagonal(
            content_to_search,
            row.try_into().unwrap(),
            (max_len - 1).try_into().unwrap(),
            Direction::RightToLeft,
        );
        count += search_overlapping(search_target, &diag);
    }

    count
}

type SearchFn = fn(&str, &[&str]) -> i32;
static SEARCH_FN_ARR: [SearchFn; 3] = [search_horizontal, search_vertical, search_diagonal];
const SEARCH_TARGET_LIST: [&str; 2] = ["XMAS", "SAMX"];

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("./input.txt")?;
    let lines: Vec<&str> = contents.lines().collect();

    let mut total_count = 0;
    for search_target in SEARCH_TARGET_LIST {
        for search_fn in SEARCH_FN_ARR {
            total_count += search_fn(search_target, &lines);
        }
    }

    println!("total \"XMAS\"\'s found : {total_count}");

    Ok(())
}
