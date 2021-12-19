use std::{fmt::Display, ops::Add};

enum Element {
    Number(i32),
    Pair(Box<Element>, Box<Element>),
}
impl Element {
    fn number(&self) -> i32 {
        if let Element::Number(n) = self {
            *n
        } else {
            panic!("not a number")
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::Pair(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

struct SnailNum(Box<Element>, Box<Element>);

impl Display for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl SnailNum {
    fn from(input: &str) -> Self {
        let mut stack = vec![];
        for c in input.chars() {
            match c {
                '[' => (),
                ',' => (),
                ']' => {
                    let r = Box::from(stack.pop().unwrap());
                    let l = Box::from(stack.pop().unwrap());
                    let el = Element::Pair(l, r);
                    stack.push(el);
                }
                _ => stack.push(Element::Number((c as u8 - '0' as u8) as i32)),
            }
        }
        if let Element::Pair(l, r) = stack.pop().unwrap() {
            SnailNum(l, r)
        } else {
            panic!("invalid num")
        }
    }

    fn reduce(&mut self) {
        loop {
            let mut done = true;
            {
                // explode
                let mut stack = vec![(1, &mut *self.1), (1, &mut *self.0)];
                let mut left_explode: Option<&mut i32> = None;
                let mut right_explode: Option<i32> = None;

                while let Some((depth, element)) = stack.pop() {
                    let mut replacement: Option<Element> = None;

                    if right_explode.is_none() {
                        if let Element::Pair(left, right) = element {
                            if depth == 4 {
                                let ln = left.number(); // XXX fails for invalid numbers ¯\_(ツ)_/¯
                                let rn = right.number();
                                if let Some(leftn) = left_explode {
                                    *leftn += ln;
                                    left_explode = None //this makes the borrow checker happy!
                                }
                                right_explode = Some(rn);
                                replacement = Some(Element::Number(0));
                                done = false;
                            }
                        }
                    }
                    if let Some(r) = replacement {
                        *element = r;
                    } else if let Element::Pair(left, right) = element {
                        // have this after the replacement thing so the borrow checker
                        // doesn't think we're borrowing element when replacing it
                        stack.push((depth + 1, &mut *right));
                        stack.push((depth + 1, &mut *left));
                    } else if let Element::Number(number) = element {
                        if let Some(rn) = right_explode {
                            *number += rn;
                            break;
                        } else {
                            left_explode = Some(number);
                        }
                    }
                }
            }

            if !done {
                continue;
            }

            {
                // split
                let mut stack = vec![(&mut *self.1), (&mut *self.0)];

                while let Some(element) = stack.pop() {
                    match element {
                        Element::Pair(left, right) => {
                            stack.push(&mut *right);
                            stack.push(&mut *left);
                        }
                        Element::Number(number) => {
                            if *number > 9 {
                                let m = *number % 2;
                                let half = (*number - m) / 2;
                                *element = Element::Pair(
                                    Box::from(Element::Number(half)),
                                    Box::from(Element::Number(half + m)),
                                );
                                done = false;
                                break;
                            }
                        }
                    }
                }
            }
            if done {
                break;
            }
        }
    }
    fn magnitude(&self) -> i64 {
        let mut stack = vec![(3, &*self.0), (2, &*self.1)];
        let mut sum = 0i64;
        while let Some((multiplier, element)) = stack.pop() {
            match element {
                Element::Pair(left, right) => {
                    stack.push((multiplier * 3, &*left));
                    stack.push((multiplier * 2, &*right));
                }
                Element::Number(number) => {
                    sum += (multiplier * number) as i64;
                }
            }
        }
        sum
    }
}

impl Add for SnailNum {
    type Output = SnailNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sn = SnailNum(
            Box::from(Element::Pair(Box::from(self.0), Box::from(self.1))),
            Box::from(Element::Pair(Box::from(rhs.0), Box::from(rhs.1))),
        );
        sn.reduce();
        sn
    }
}

pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/18.txt")?;
    let lines = data.lines().collect::<Vec<_>>();

    let result1 = lines
        .iter()
        .map(|&s| SnailNum::from(s))
        .reduce(|acc, num| acc + num)
        .unwrap()
        .magnitude();

    println!("puzzle 18.1 {}", result1);

    let result2 = (0..lines.len())
        .map(|a| (0..lines.len()).map(move |b| (a, b)))
        .flatten()
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (SnailNum::from(lines[a]) + SnailNum::from(lines[b])).magnitude())
        .reduce(i64::max)
        .unwrap();

    println!("puzzle 18.2 {}", result2);

    Ok(())
}
