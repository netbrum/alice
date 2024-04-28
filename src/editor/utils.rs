use super::{Line, LINE_NUMBER_COLUMN_GAP};

pub fn digits(number: usize) -> usize {
    number.checked_ilog10().unwrap_or(0) as usize + 1
}

pub fn ln_offset(lines: &[Line]) -> usize {
    digits(lines.len()) + LINE_NUMBER_COLUMN_GAP
}
