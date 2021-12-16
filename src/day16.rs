macro_rules! ones {
    ($value:expr) => {
        (2u32.pow($value as u32) - 1) as u8
    };
}

pub fn run() -> std::io::Result<()> {
    let input = std::fs::read_to_string("data/16.txt")?;
    let data = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect::<Vec<_>>();

    let mut loc = 0;

    let mut take = |bits: usize| -> u64 {
        let mut val = 0u64;
        let offset = loc % 8;
        let mut curr = data[(loc - offset) / 8] & ones!(8 - offset);
        let mut read = 8 - offset;
        while read < bits {
            val += curr as u64;
            curr = data[(loc + read) / 8];
            read += 8;
            val <<= 8;
        }
        loc += bits;
        (val + curr as u64) >> read - bits
    };

    // loop {
    //     let version = take(3);
    //     let typeid = take(3);
    // }

    // println!("{:b}", take(5));
    // println!("{:b}", take(5));
    // println!("{:b}", take(5));
    // println!("{:b}", take(5));

    println!("puzzle 15.1 {}", "solve(&grid, 1)");

    Ok(())
}
