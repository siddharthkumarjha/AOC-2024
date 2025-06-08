/// find two MAS in the shape of an X. One way to achieve that is like this:
///
/// M.S
/// .A.
/// M.S
///
/// Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.
///
/// Here's the same example from before, but this time all of the X-MASes have been kept instead:
///
/// .M.S......
/// ..A..MSMS.
/// .M.S.MAA..
/// ..A.ASMSM.
/// .M.S.M....
/// ..........
/// S.S.S.S.S.
/// .A.A.A.A..
/// M.M.M.M.M.
/// ..........
///
/// In this example, an X-MAS appears 9 times.
///
/// Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?
use std::error::Error;
use std::fs;

fn match_blk_at(target: &[&str], content: &[&str], start_row: usize, start_col: usize) -> bool {
    for (row_offset, target_row) in target.iter().enumerate() {
        for (col_offset, target_char) in target_row.chars().enumerate() {
            if target_char == '.' {
                continue;
            }

            let cur_content_char = content[row_offset + start_row]
                .chars()
                .nth(col_offset + start_col)
                .unwrap();

            if cur_content_char != target_char {
                return false;
            }
        }
    }
    true
}

fn count_blk_matches(target: &[&str], content: &[&str]) -> u32 {
    let content_rows = content.len();
    let content_cols = content.iter().map(|col| col.len()).max().unwrap();

    let target_rows = target.len();
    let target_cols = target.iter().map(|col| col.len()).max().unwrap();

    let mut count = 0;

    let window_row = content_rows - target_rows + 1;
    let window_col = content_cols - target_cols + 1;
    for row in 0..window_row {
        for col in 0..window_col {
            if match_blk_at(target, content, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("./input.txt")?;
    let lines: Vec<&str> = contents.lines().collect();
    let target_list: Vec<Vec<&str>> = vec![
        "\
M.S
.A.
M.S"
        .lines()
        .collect(),
        "\
S.M
.A.
S.M"
        .lines()
        .collect(),
        "\
M.M
.A.
S.S"
        .lines()
        .collect(),
        "\
S.S
.A.
M.M"
        .lines()
        .collect(),
    ];

    let mut total_count = 0;
    for target in target_list {
        total_count += count_blk_matches(&target, &lines);
    }

    println!("total \"XMAS\"\'s found : {total_count}");

    Ok(())
}
