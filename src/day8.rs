use std::collections::HashMap;

struct Permuter<'a, T> {
    state: Vec<&'a T>,
    counters: Vec<u8>,
    done: bool,
}

impl<'a, T> Permuter<'a, T> {
    fn new(input: &'a Vec<T>) -> Self {
        Permuter {
            state: input.iter().collect::<Vec<_>>(),
            counters: vec![0; input.len() - 1],
            done: false,
        }
    }
}

impl<'a, T> Iterator for Permuter<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            // https://en.wikipedia.org/wiki/Heap%27s_algorithm
            for (idx, c) in self.counters.iter_mut().enumerate() {
                if *c > (idx as u8) {
                    *c = 0;
                } else {
                    let value = self.state.clone();
                    if idx % 2 == 1 {
                        self.state.swap(0, idx + 1)
                    } else {
                        self.state.swap(*c as usize, idx + 1)
                    }
                    *c += 1;
                    return Some(value);
                }
            }
            self.done = true;
            Some(self.state.clone())
        }
    }
}

fn translate(mapping: &Vec<&u8>, input: &str) -> String {
    let base = 'a' as u8;
    let mut chars = input
        .chars()
        .map(|c| (mapping[(c as u8 - base) as usize] + base) as char)
        .collect::<Vec<_>>();
    chars.sort();
    String::from_iter(chars.iter())
}

pub fn run() -> std::io::Result<()> {
    let digits = HashMap::from([
        ("cf", 1),
        ("acf", 7),
        ("bcdf", 4),
        ("acdeg", 2),
        ("acdfg", 3),
        ("abdfg", 5),
        ("abcefg", 0),
        ("abdefg", 6),
        ("abcdfg", 9),
        ("abcdefg", 8),
    ]);

    let indices = vec![0u8, 1, 2, 3, 4, 5, 6];

    let data = std::fs::read_to_string("data/8.txt")?
        .lines()
        .map(|l| {
            let (nums, output) = l.split_once(" | ").unwrap();
            (
                nums.split(" ").collect::<Vec<_>>(),
                output.split(" ").collect::<Vec<_>>(),
            )
        })
        .map(|(input, output)| -> Vec<i32> {
            for permutation in Permuter::new(&indices) {
                if input
                    .iter()
                    .map(|s| translate(&permutation, s))
                    .all(|s| digits.contains_key(&s as &str))
                {
                    // println!("{:?}", permutation);
                    return output
                        .iter()
                        .map(|s| translate(&permutation, s))
                        .map(|s| *digits.get(&s as &str).unwrap())
                        .collect::<Vec<_>>();
                }
            }
            panic!("no permutation found")
        })
        .collect::<Vec<_>>();

    let result1 = data
        .iter()
        .map(|v| v.iter())
        .flatten()
        .filter(|&&d| d == 1 || d == 4 || d == 7 || d == 8)
        .count();

    println!("puzzle 8.1 {}", result1);

    let result2 = data
        .iter()
        .map(|v| {
            v.iter().rev().enumerate().fold(0, |acc, (power, dgt)| {
                acc + 10u32.pow(power as u32) as i32 * dgt
            })
        })
        .sum::<i32>();

    println!("puzzle 8.2 {}", result2);

    Ok(())
}
