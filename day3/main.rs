fn main() {

    let lines:Vec<_> = std::fs::read_to_string("./input.txt")
        .expect("could not load file")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("bad data")
                })
                .collect::<Vec<bool>>()
        })
        .collect();
    
    let pt1 = slope(&lines, 3, 1);
    
    let pt2 = slope(&lines, 3, 1) * slope(&lines, 1, 1) * slope(&lines, 5, 1) * slope(&lines, 7, 1) * slope(&lines, 1, 2);

    println!("{}", pt2);
}

fn slope(data: &Vec<Vec<bool>>, right: usize, down: usize) -> usize {
    let row_len = data[0].len();

    (0..data.len())
        .step_by(down)
        .enumerate()
        .map(|(col, row)| (row, (col * right) % row_len))
        .filter(|(row, col)| data[*row][*col])
        .count()
}