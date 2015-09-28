fn main() {
    let rows: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K'];
    let columns: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9,  10];

    let grid = build_grid(&rows, &columns);
}

fn build_grid(rows: &[char; 10], columns: &[i32; 10]) {
    let line = std::iter::repeat("------").take(10).collect::<String>().to_string();

    display_upper_row(&line, *columns);
}

fn display_upper_row(line: &str, columns: [i32; 10]) {
    let mut parts: Vec<String> = vec![];
    for column in &columns {
        parts.push(column.to_string());
    }
    let mut column_part = format!("|  {}  |", parts.join("  |  ")); // String, because it needs to be growable

    println!("{}", line);
    println!("{}", column_part);
    println!("{}", line);
}
