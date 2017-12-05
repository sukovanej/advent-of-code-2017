fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err input");

    let mut sum = 0u32;
    let mut last = 0;

    for ch in input.chars() {
        if ch == '\n' { continue; }

        let val = ch.to_string().parse::<u32>().unwrap();

        if last == val {
            sum += last;
        }

        last = val;
    }

    let bytes = input.chars().collect::<Vec<char>>();
    let mut last_index = bytes.len() - 1;

    while !bytes[last_index].is_digit(10) {
        last_index -= 1;
    }

    if bytes[0] == bytes[last_index] {
        sum += bytes[0].to_string().parse::<u32>().unwrap();
    }

    println!("{}", sum);
}