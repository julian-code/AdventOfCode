struct Cli {
    path: std::path::PathBuf
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli {
        path: std::path::PathBuf::from(path)
    };
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        let current_num : u32 = line.parse().unwrap();
        for line in content.lines() {
            let other_num : u32 = line.parse().unwrap();
            if current_num + other_num == 2020 {
                println!("{} * {} = {}", current_num, other_num, current_num * other_num);
            }
        }
    }
    
}