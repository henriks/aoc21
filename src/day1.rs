pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/1_1.txt")?;

    let mut count = -1;
    let mut prev = 0;

    let mut sums = [0, 0, 0];
    let mut count2 = 0;
    let mut prev_sum: i32;

    for (counter, line) in data.lines().enumerate() {
        let num: i32 = line.parse().unwrap();
        if num > prev {
            count += 1;
        }
        prev = num;

        prev_sum = sums.iter().sum();
        sums[counter % 3] = num;

        if counter > 2 {
            let curr_sum: i32 = sums.iter().sum();
            if curr_sum > prev_sum {
                count2 += 1;
            }
        }
    }

    println!("puzzle1 {}", count);
    println!("puzzle2 {}", count2);

    Ok(())
}
