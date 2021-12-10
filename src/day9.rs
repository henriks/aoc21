use std::collections::HashSet;

pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/9.txt")?
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dimx = 100;
    let dimy = 100;

    let neighbors = |(y, x): (usize, usize)| {
        let mut out = vec![];
        if x > 0 {
            out.push((y, x - 1))
        }
        if x < dimx - 1 {
            out.push((y, x + 1))
        }
        if y > 0 {
            out.push((y - 1, x))
        }
        if y < dimy - 1 {
            out.push((y + 1, x))
        }
        out
    };

    let points = (0..dimy)
        .map(|y| (0..dimx).map(move |x| (y, x)))
        .flatten()
        .filter(|(y, x)| {
            let v: u8 = data[*y][*x];
            neighbors((*y, *x))
                .iter()
                .all(|(y1, x1)| data[*y1][*x1] > v)
        })
        .collect::<Vec<_>>();

    let result1 = points
        .iter()
        .map(|(y, x)| data[*y][*x] as i32 + 1)
        .sum::<i32>();

    println!("puzzle 9.1 {}", result1);

    let mut basins = points
        .iter()
        .map(|(y, x)| {
            let mut basin = HashSet::from([(*y, *x)]);
            let mut front = basin.clone();

            while !front.is_empty() {
                let mut next = HashSet::new();
                front.iter().for_each(|(y, x)| {
                    neighbors((*y, *x)).iter().for_each(|&(y1, x1)| {
                        if !basin.contains(&(y1, x1)) && data[y1][x1] < 9 {
                            next.insert((y1, x1));
                            basin.insert((y1, x1));
                        }
                    })
                });

                front = next;
            }

            basin
        })
        .collect::<Vec<_>>();

    basins.sort_by_key(|b| b.len());
    basins.reverse();

    let result2 = basins[0..3].iter().fold(1, |acc, b| acc * b.len());
    println!("puzzle 9.2 {}", result2);

    Ok(())
}
