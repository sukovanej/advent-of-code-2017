fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err input");

    let data: Vec<Vec<u32>> = input
        .split('\n')
        .map(|x| x
            .split(' ')
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
        .collect();

    data.iter().for_each(|x| x.sort());
}