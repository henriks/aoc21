use std::collections::HashMap;

pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/14.txt")?;

    let mut lines = data.lines();
    let mut initial_pairs = HashMap::new();

    let template = lines.next().unwrap();
    template
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .for_each(|pair| *(initial_pairs.entry(pair).or_insert(0u64)) += 1);
    lines.next();

    let count = |data: &HashMap<(char, char), u64>| {
        let mut counts = data
            .iter()
            .fold(HashMap::new(), |mut acc, (&(a, b), count)| {
                *(acc.entry(a).or_insert(0)) += count;
                *(acc.entry(b).or_insert(0)) += count;
                acc
            });
        let first = template.chars().nth(0).unwrap();
        let last = template.chars().last().unwrap();
        for (c, count) in &mut counts {
            if *c == first {
                *count += 1;
            }
            if *c == last {
                *count += 1;
            }
            *count /= 2;
        }
        let (max, min) = counts.iter().fold((0, u64::MAX), |(max, min), (_, count)| {
            (max.max(*count), min.min(*count))
        });
        max - min
    };

    let rules = lines
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            (
                (from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()),
                to.chars().nth(0).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let calc = |pairs: HashMap<(char, char), u64>, _| {
        let mut next = HashMap::new();
        rules.iter().for_each(|&(key, c)| {
            pairs.get(&key).iter().for_each(|&&amount| {
                *(next.entry((key.0, c)).or_insert(0)) += amount;
                *(next.entry((c, key.1)).or_insert(0)) += amount;
            });
        });
        next
    };

    let pairs1 = (0..10).fold(initial_pairs.clone(), calc);
    println!("puzzle 14.1 {}", count(&pairs1));

    let pairs2 = (0..40).fold(initial_pairs.clone(), calc);
    println!("puzzle 14.2 {}", count(&pairs2));

    Ok(())
}
