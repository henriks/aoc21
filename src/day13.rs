pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/13.txt")?;

    let mut lines = data.lines();

    let mut coords = vec![];
    let mut folds = vec![];

    let mut xmax = 0;
    let mut ymax = 0;

    while let Some(l) = lines.next() {
        if l == "" {
            break;
        }
        let (xs, ys) = l.split_once(",").unwrap();
        let (x, y) = (xs.parse::<usize>().unwrap(), ys.parse::<usize>().unwrap());
        xmax = xmax.max(x);
        ymax = ymax.max(y);
        coords.push((x, y));
    }

    while let Some(l) = lines.next() {
        let (axis, coord) = l[11..].split_once("=").unwrap();
        folds.push((axis, coord.parse::<usize>().unwrap()));
    }

    let mut grid = vec![vec![false; xmax + 1]; ymax + 1];
    coords.iter().for_each(|&(x, y)| grid[y][x] = true);

    folds.iter().enumerate().for_each(|(idx, &(axis, coord))| {
        match axis {
            "x" => {
                ((coord + 1)..=xmax).for_each(|x| {
                    (0..=ymax).for_each(|y| {
                        grid[y][2 * coord - x] |= grid[y][x];
                    })
                });
                xmax = coord - 1;
            }
            "y" => {
                ((coord + 1)..=ymax).for_each(|y| {
                    (0..=xmax).for_each(|x| {
                        grid[2 * coord - y][x] |= grid[y][x];
                    })
                });
                ymax = coord - 1;
            }
            _ => panic!("invalid axis"),
        };
        if idx == 0 {
            println!(
                "puzzle 13.1 {}",
                grid.iter()
                    .take(ymax + 1)
                    .map(|r| r.iter().take(xmax + 1))
                    .flatten()
                    .filter(|&&dot| dot)
                    .count()
            );
        }
    });

    println!("puzzle 13.2");
    for l in grid.iter().take(ymax + 1) {
        println!(
            "{}",
            l[0..=xmax]
                .iter()
                .map(|&c| if c { "#" } else { "." })
                .collect::<String>()
        );
    }

    Ok(())
}
