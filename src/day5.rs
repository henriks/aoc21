fn parse_tuple(tpl: &str) -> (i32, i32) {
    let (xstr, ystr) = tpl.split_once(",").unwrap();
    (xstr.parse::<i32>().unwrap(), ystr.parse::<i32>().unwrap())
}

pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/5.txt")?
        .lines()
        .map(|s| {
            let (from, to) = s.split_once(" -> ").unwrap();
            (parse_tuple(from), parse_tuple(to))
        })
        .collect::<Vec<_>>();

    let dim = data.iter().fold((0, 0), |(x, y), (from, to)| {
        (
            x.max(from.0 + 1).max(to.0 + 1),
            y.max(from.1 + 1).max(to.1 + 1),
        )
    });

    // ranges are painful, who knew

    let result = data
        .iter()
        .filter(|((x0, y0), (x1, y1))| x0 == x1 || y0 == y1)
        .fold(
            vec![0; (dim.0 * dim.1) as usize],
            |mut board, ((x0, y0), (x1, y1))| {
                if x0 == x1 {
                    for y in if *y0 < *y1 {
                        (*y0)..=*(y1)
                    } else {
                        (*y1)..=(*y0)
                    } {
                        board[(dim.0 * y + x0) as usize] += 1;
                    }
                } else if y0 == y1 {
                    for x in if *x0 < *x1 {
                        (*x0)..=*(x1)
                    } else {
                        (*x1)..=(*x0)
                    } {
                        board[(dim.0 * y0 + x) as usize] += 1;
                    }
                }
                board
            },
        )
        .iter()
        .filter(|&&i| i >= 2)
        .count();

    println!("puzzle 5.1 {}", result);

    let result2 = data
        .iter()
        .filter(|((x0, y0), (x1, y1))| x0 == x1 || y0 == y1 || (x0 - x1).abs() == (y0 - y1).abs())
        .fold(
            vec![0; (dim.0 * dim.1) as usize],
            |mut board, ((x0, y0), (x1, y1))| {
                if x0 == x1 {
                    for y in if *y0 < *y1 {
                        (*y0)..=*(y1)
                    } else {
                        (*y1)..=(*y0)
                    } {
                        board[(dim.0 * y + x0) as usize] += 1;
                    }
                } else if y0 == y1 {
                    for x in if *x0 < *x1 {
                        (*x0)..=*(x1)
                    } else {
                        (*x1)..=(*x0)
                    } {
                        board[(dim.0 * y0 + x) as usize] += 1;
                    }
                } else {
                    let yrng: Vec<i32> = if *y0 < *y1 {
                        ((*y0)..=*(y1)).collect()
                    } else {
                        ((*y1)..=(*y0)).rev().collect()
                    };
                    let xrng: Vec<i32> = if *x0 < *x1 {
                        ((*x0)..=*(x1)).collect()
                    } else {
                        ((*x1)..=(*x0)).rev().collect()
                    };
                    for (x, y) in xrng.iter().zip(yrng.iter()) {
                        board[(dim.0 * y + x) as usize] += 1;
                    }
                }
                board
            },
        )
        .iter()
        .filter(|&&i| i >= 2)
        .count();

    println!("puzzle 5.2 {}", result2);

    Ok(())
}
