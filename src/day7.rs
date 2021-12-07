pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/7.txt")?
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let result = data
        .iter()
        .map(|pos| data.iter().fold(0, |acc, n| acc + (n - pos).abs()))
        .min()
        .unwrap();

    println!("puzzle 7.1 {}", result);

    let min = data.iter().min().unwrap();
    let max = data.iter().max().unwrap();

    let result2 = (*min..=*max)
        .map(|pos| {
            data.iter().fold(0i64, |acc, n| {
                let x = (n - pos).abs();
                let cost = x * (x + 1); // cost is off by 2x
                acc + cost
            })
        })
        .min()
        .unwrap()
        / 2;

    println!("puzzle 7.2 {}", result2);

    Ok(())
}
