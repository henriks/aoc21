use std::collections::HashSet;

#[derive(Debug)]
struct Board(Vec<u32>);

impl Board {
    fn bingo(&self, num: u32, drawn: &HashSet<u32>) -> Option<u32> {
        match self.has(num) {
            Some((x, y)) => {
                if (0..5).all(|y_| (&drawn).contains(&self.at(x, y_)))
                    || (0..5).all(|x_| (&drawn).contains(&self.at(x_, y)))
                // || ((x == y) && (0..4).all(|x_| (&drawn).contains(&self.at(x_, x_))))
                // || ((4 - x == y) && (0..4).all(|x_| (&drawn).contains(&self.at(4 - x_, x_))))
                {
                    let sum: u32 = self.0.iter().filter(|n| !drawn.contains(n)).sum();
                    Some(sum * num)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn has(&self, num: u32) -> Option<(u8, u8)> {
        self.0.iter().position(|&n| n == num).map(|pos| {
            let x = pos % 5;
            let y = (pos - x) / 5;
            (x as u8, y as u8)
        })
    }
    fn at(&self, x: u8, y: u8) -> u32 {
        self.0[(y * 5 + x) as usize]
    }
}

pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/4.txt")?;

    let lines = data.lines().collect::<Vec<_>>();

    let nums = lines[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = (&lines[1..])
        .chunks(6)
        .map(|rows| {
            let boardnums = rows[1..6]
                .iter()
                .map(|r| {
                    r.as_bytes()
                        .chunks(3)
                        .map(|c| std::str::from_utf8(c).unwrap())
                        .map(|s| s.trim().parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            Board(boardnums)
        })
        .collect::<Vec<_>>();

    let mut drawn = HashSet::new();

    'outer: for num in &nums {
        drawn.insert(*num);
        for board in &boards {
            if let Some(score) = board.bingo(*num, &drawn) {
                println!("puzzle 4.1 {:?}", score);
                break 'outer;
            }
        }
    }

    drawn = HashSet::new();
    let mut remaining = boards.iter().collect::<Vec<_>>();

    'outer2: for num in &nums {
        drawn.insert(*num);
        let mut purge = Vec::new();
        for (idx, &board) in remaining.iter().enumerate() {
            if let Some(score) = board.bingo(*num, &drawn) {
                if remaining.len() == 1 {
                    println!("puzzle 4.2 {:?}", score);
                    break 'outer2;
                } else {
                    purge.push(idx);
                }
            }
        }

        for p in purge.iter().rev() {
            remaining.remove(*p);
        }
    }

    Ok(())
}
