use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug)]
struct Step {
    curr: (usize, usize),
    prev: (usize, usize),
    cost: u32,
    approx: u32,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.curr == other.curr
            && self.prev == other.prev
            && self.cost == other.cost
            && self.approx == other.approx
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Step {}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost + self.approx).cmp(&(other.cost + other.approx))
    }
}

fn solve(grid: &Vec<Vec<u8>>, multiplier: usize) -> u32 {
    let mut fringe: BinaryHeap<Reverse<Step>> = BinaryHeap::new();
    let mut trails: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let gy = grid.len();
    let gx = grid[0].len();

    let ymax = (gy * multiplier) - 1;
    let xmax = (gx * multiplier) - 1;

    fringe.push(Reverse(Step {
        curr: (0, 0),
        prev: (0, 0),
        cost: 0,
        approx: (ymax + xmax) as u32,
    }));

    let cost = |y: usize, x: usize| {
        let y1 = y % gy;
        let x1 = x % gx;

        let ymult = (y - y1) / gy;
        let xmult = (x - x1) / gx;

        ((grid[y1][x1] as usize - 1 + ymult + xmult) % 9) as u32 + 1
    };

    loop {
        if let Some(Reverse(next)) = fringe.pop() {
            if trails.contains_key(&next.curr) {
                continue;
            }

            trails.insert(next.curr, next.prev);

            if next.curr == (ymax, xmax) {
                break next.cost;
            }

            for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (y, x) = next.curr;
                let x1 = x as i32 + dx;
                let y1 = y as i32 + dy;
                if x1 >= 0 && x1 as usize <= xmax && y1 >= 0 && y1 as usize <= ymax {
                    let tpl = (y1 as usize, x1 as usize);
                    if !trails.contains_key(&tpl) {
                        fringe.push(Reverse(Step {
                            curr: tpl,
                            prev: next.curr,
                            cost: next.cost + cost(tpl.0, tpl.1),
                            approx: ((ymax - tpl.0) + (xmax - tpl.1)) as u32,
                        }));
                    }
                }
            }
        } else {
            panic!("ran out of steps")
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/15.txt")?;

    let grid = data
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| c - '0' as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("puzzle 15.1 {}", solve(&grid, 1));
    println!("puzzle 15.2 {}", solve(&grid, 5));

    Ok(())
}
