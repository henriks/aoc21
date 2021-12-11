pub fn run() -> std::io::Result<()> {
    let mut initial_data = [[0; 10]; 10];
    std::fs::read_to_string("data/11.txt")?
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            l.chars()
                .enumerate()
                .for_each(|(x, c)| initial_data[y][x] = (c as u8 - '0' as u8) as i32)
        });

    let mut data = initial_data.clone();

    let coords = || (0..10).flat_map(|y| (0..10).map(move |x| (y, x)));
    let neighbors = |y, x| {
        [
            (0i32, 1i32),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ]
        .iter()
        .map(|&(y1, x1)| (y as i32 + y1, x as i32 + x1))
        .filter(|&(y1, x1)| y1 >= 0 && y1 < 10 && x1 >= 0 && x1 < 10)
        .map(|(y1, x1)| (y1 as usize, x1 as usize))
        .collect::<Vec<(usize, usize)>>()
    };

    let mut flashes = 0;
    for _ in 0..100 {
        coords().for_each(|(y, x)| data[y][x] += 1);
        loop {
            let mut loop_flashes = 0;
            coords().for_each(|(y, x)| {
                if data[y][x] > 9 {
                    data[y][x] = 0;
                    loop_flashes += 1;
                    neighbors(y, x).iter().for_each(|&(y1, x1)| {
                        if data[y1][x1] != 0 {
                            data[y1][x1] += 1
                        }
                    })
                }
            });
            if loop_flashes == 0 {
                break;
            } else {
                flashes += loop_flashes;
            }
        }
    }

    println!("puzzle 10.1 {}", flashes);

    let mut result2 = 0;
    data = initial_data;
    loop {
        coords().for_each(|(y, x)| data[y][x] += 1);
        let mut step_flashes = 0;
        loop {
            let mut loop_flashes = 0;
            coords().for_each(|(y, x)| {
                if data[y][x] > 9 {
                    data[y][x] = 0;
                    loop_flashes += 1;
                    neighbors(y, x).iter().for_each(|&(y1, x1)| {
                        if data[y1][x1] != 0 {
                            data[y1][x1] += 1
                        }
                    })
                }
            });
            if loop_flashes == 0 {
                break;
            } else {
                step_flashes += loop_flashes;
            }
        }
        result2 += 1;
        if step_flashes == 100 {
            break;
        }
    }
    println!("puzzle 10.2 {}", result2);

    Ok(())
}
