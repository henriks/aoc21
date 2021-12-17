pub fn run() -> std::io::Result<()> {
    let xmin = 155;
    let xmax = 182;
    let ymax = -67;
    let ymin = -117;

    let xfeasible = |xvel1: i32| {
        let mut x = 0;
        let mut xvel = xvel1;
        loop {
            x += xvel;
            xvel -= 1;
            if x >= xmin {
                break x <= xmax;
            } else if x == 0 {
                break false;
            }
        }
    };

    let yfeasible = |yvel1: i32| {
        let mut y = 0;
        let mut yvel = yvel1;
        loop {
            y += yvel;
            yvel -= 1;
            if y <= ymax {
                break y >= ymin;
            }
        }
    };

    let hits = |xvel1: i32, yvel1: i32| {
        let mut x = 0;
        let mut y = 0;
        let mut xvel = xvel1;
        let mut yvel = yvel1;
        let mut points = vec![(x, y)];
        loop {
            x += xvel;
            if xvel > 0 {
                xvel -= 1;
            }
            y += yvel;
            yvel -= 1;
            points.push((x, y));
            if x >= xmin && x <= xmax && y <= ymax && y >= ymin {
                break Some(points);
            } else if x > xmax || y < ymin {
                break None;
            }
        }
    };

    let xrange = (((-1 + ((1 + 8 * xmin) as f64).sqrt().ceil() as i32) / 2)..=xmax)
        .filter(|&x| xfeasible(x))
        .collect::<Vec<_>>();

    let yrange = (ymin..1000).filter(|&y| yfeasible(y)).collect::<Vec<_>>();

    let result = yrange
        .iter()
        .rev()
        .map(|&y| {
            xrange
                .iter()
                .map(|&x| hits(x, y))
                .flatten()
                .collect::<Vec<_>>()
        })
        .filter(|r| !r.is_empty())
        .collect::<Vec<_>>();

    println!(
        "puzzle 17.1 {}",
        result[0][0].iter().map(|tpl| tpl.1).max().unwrap()
    );

    println!(
        "puzzle 17.2 {}",
        result.iter().map(|r| r.len()).sum::<usize>()
    );

    Ok(())
}
