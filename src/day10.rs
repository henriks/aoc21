pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/10.txt")?;
    let lines = data.lines().collect::<Vec<_>>();

    let result1 = lines
        .iter()
        .map(|l| {
            let mut expected = vec![];
            let mut errorscore = 0;
            for c in l.chars() {
                match c {
                    '(' => expected.push(')'),
                    '[' => expected.push(']'),
                    '{' => expected.push('}'),
                    '<' => expected.push('>'),
                    other => {
                        if let Some(e) = expected.pop() {
                            if other == e {
                                continue;
                            }
                        }
                        errorscore = match other {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        }
                    }
                }
            }
            errorscore
        })
        .sum::<i32>();

    println!("puzzle 10.1 {}", result1);

    let mut scores = lines
        .iter()
        .map(|l| {
            let mut expected = vec![];
            for c in l.chars() {
                match c {
                    '(' => expected.push(')'),
                    '[' => expected.push(']'),
                    '{' => expected.push('}'),
                    '<' => expected.push('>'),
                    other => {
                        if let Some(e) = expected.pop() {
                            if other == e {
                                continue;
                            }
                        }
                        return None;
                    }
                }
            }
            let score = expected.iter().rev().fold(0i64, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
            });
            Some(score)
        })
        .flatten()
        .collect::<Vec<_>>();

    scores.sort();
    let result2 = scores[(scores.len() - 1) / 2];

    println!("puzzle 10.2 {}", result2);

    Ok(())
}
