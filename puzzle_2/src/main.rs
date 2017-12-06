fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err input");
    let sum = resolve(input);

    println!("{}", sum);
}

fn resolve(input: String) -> u32 {
    let mut sum = 0u32;
    let chars = input.chars().filter(|x| *x != '\n').collect::<Vec<char>>();
    let mut i = chars.len() / 2;

    for a in chars.iter() {
        let val_a = a.to_string().parse::<u32>().unwrap();
        let val_b = chars[i].to_string().parse::<u32>().unwrap();

        if val_a == val_b {
            sum += val_a;
        }

        i += 1;
        if i == chars.len() {
            i = 0;
        }
    }

    return sum;
}

#[test]
fn test_1() {
    assert_eq!(resolve("1212".to_string()), 6);
    assert_eq!(resolve("1221".to_string()), 0);
    assert_eq!(resolve("123425".to_string()), 4);
    assert_eq!(resolve("123123".to_string()), 12);
    assert_eq!(resolve("12131415".to_string()), 4);
}