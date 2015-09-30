fn main() {
    let rows: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K'];
    let amount_of_columns = 10;

    let grid = build_grid(&rows, amount_of_columns);
}

fn build_grid(rows: &[char; 10], amount_of_columns: i32) {
    // take() takes a usize, so we need to cast the i32 to a usize: http://stackoverflow.com/a/28601386
    let line = std::iter::repeat("------").take(amount_of_columns as usize)
                                          .collect::<String>()
                                          .to_string();

    display_upper_row(&line, amount_of_columns);
}

fn display_upper_row(line: &str, amount_of_columns: i32) {
    let mut parts: Vec<String> = vec![];
    for column in (1..amount_of_columns + 1) {
        parts.push(column.to_string());
    }
    let column_part = format!("|  {}  |", parts.join("  |  ")); // String, because it needs to be growable

    println!("{}", line);
    println!("{}", column_part);
    println!("{}", line);
}
