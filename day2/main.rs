struct Password<'a> {
    policy: Policy,
    value: &'a str
}

struct Policy {
    character: char,
    first_index: usize,
    last_index: usize
}

impl<'a> Password<'a> {
    fn new(x: &'a str) -> Self {
        let v: Vec<&str> = x.split_whitespace().collect();
        let lower_upper: Vec<&'a str> = v[0].split('-').collect();
        Self {
            policy: Policy {
                character: v[1].chars().nth(0).unwrap(),
                first_index: lower_upper[0].parse().unwrap(),
                last_index: lower_upper[1].parse().unwrap()
            },
            value: v[2]
        }
    }
}

impl<'a> Password<'a> {
    fn is_valid(&self) -> bool {
        let first_char:char = self.value.chars().nth(self.policy.first_index - 1).unwrap();
        let second_char:char = self.value.chars().nth(self.policy.last_index - 1).unwrap();
        let first_char_valid:bool = first_char == self.policy.character;
        let second_char_valid:bool = second_char == self.policy.character;

        first_char_valid ^ second_char_valid
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    println!("{}", path);
    let input = std::fs::read_to_string(path)
        .expect("could not read file");

    let valid_passwords:Vec<Password> = input.lines().map(|x| Password::new(x)).filter(|x| x.is_valid()).collect();

    println!("{}", valid_passwords.len());
}