fn readdata() -> std::io::Result<Vec<u8>> {
    let data = std::fs::read_to_string("data/6.txt")?
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    Ok(data)
}

pub fn run() -> std::io::Result<()> {
    let mut data = readdata()?;
    for _ in 0..80 {
        let mut new = 0;
        for fish in &mut data {
            if *fish == 0 {
                *fish = 6;
                new += 1;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..new {
            data.push(8);
        }
    }

    println!("puzzle 6.1 {}", data.len());

    data = readdata()?;
    let mut counts = [0u64; 9];

    for d in &data {
        counts[*d as usize] += 1;
    }

    for _ in 0..256 {
        let zeros = counts[0];
        for i in 0..8 {
            counts[i] = counts[i + 1];
        }
        counts[6] += zeros;
        counts[8] = zeros;
    }

    println!("puzzle 6.2 {}", counts.iter().sum::<u64>());

    Ok(())
}
