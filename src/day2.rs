pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/2.txt")?;

    let processed = data
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(dir, count)| {
            let n: i32 = count.parse().unwrap();
            match dir {
                "up" => (0, -n),
                "down" => (0, n),
                "forward" => (n, 0),
                _ => (0, 0),
            }
        })
        .collect::<Vec<_>>();

    let result1 = processed
        .iter()
        .fold((0, 0), |(x0, y0), (x, y)| (x0 + x, y0 + y));

    println!("puzzle 2.1 {}", result1.0 * result1.1);

    let result2 = processed.iter().fold((0, 0, 0), |(x0, y0, a0), (x, y)| {
        (x0 + x, y0 + (x * a0), a0 + y)
    });

    println!("puzzle 2.2 {}", result2.0 * result2.1);

    Ok(())
}
