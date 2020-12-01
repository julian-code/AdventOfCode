fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut numbers:Vec<u32> = std::fs::read_to_string(path)
        .expect("could not read file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.sort();

    'outer: for (ix, n0) in numbers.iter().enumerate() {
        for n1 in numbers[(ix+1)..].iter() {
            for n2 in numbers[(ix+2)..].iter() {
                if n0 + n1 + n2 == 2020 {
                    println!("{} + {} + {} = 2020", n0, n1, n2);
                    println!("{} * {} * {} = {}", n0, n1, n2, n0 * n1 * n2);
                    break 'outer;
                }
            }
        }
    }
}