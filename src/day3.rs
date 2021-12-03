pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/3.txt")?;
    let lines = data
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // I'm probably doing this wrong
    let linerefs = lines.iter().map(|s| s.as_ref()).collect::<Vec<_>>();

    let result1 = counts(&linerefs)
        .iter()
        .map(|dgt| if *dgt < 0 { '0' } else { '1' })
        .collect::<String>();

    let gamma = u32::from_str_radix(&result1, 2).unwrap();
    let epsilon = gamma ^ 0b111111111111;

    println!("puzzle 3.1 {:?}", gamma * epsilon);

    fn filter(mut remaining: Vec<&[char]>, test: fn(char, i32) -> bool) -> u32 {
        let mut idx = 0;

        loop {
            if remaining.len() > 1 {
                let c = counts(&remaining);
                remaining = remaining
                    .into_iter()
                    .filter(|row| test(row[idx], c[idx]))
                    .collect::<Vec<_>>()
            } else {
                break u32::from_str_radix(&remaining[0].iter().collect::<String>(), 2).unwrap();
            }
            idx += 1;
        }
    }

    let oxy = filter(linerefs.clone(), |c, avg| (c == '1') ^ (avg < 0));
    let co2 = filter(linerefs.clone(), |c, avg| (c == '0') ^ (avg < 0));

    println!("puzzle 3.2 {:?}", oxy * co2);

    Ok(())
}

fn counts(ls: &[&[char]]) -> Vec<i32> {
    ls.iter().fold(vec![0; 12], |curr, chars| {
        curr.iter()
            .zip(chars.iter())
            .map(|(num, c)| num + if *c == '0' { -1 } else { 1 })
            .collect()
    })
}
