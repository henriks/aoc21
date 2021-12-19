use std::{fmt::Display, ops::Deref};

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
        // parent loop
        loop {
            let mut done = true;
            // explode loop
            {
                let mut stack = vec![(1, &mut *self.1), (1, &mut *self.0)];
                let mut left_number: Option<&mut i32> = None;
                let mut right_number: Option<i32> = None;

                while let Some((depth, element)) = stack.pop() {
                    println!("{} -> {}", depth, element);
                    let mut replacement: Option<Element> = None;

                    if right_number.is_none() {
                        if let Element::Pair(left, right) = element {
                            if depth == 4 {
                                let ln = left.number(); // XXX fails for invalid numbers ¯\_(ツ)_/¯
                                let rn = right.number();
                                if let Some(leftn) = left_number {
                                    println!("adding to left {}: {}", leftn, ln);
                                    *leftn += ln;
                                    left_number = None //this makes the borrow checker happy!
                                }
                                right_number = Some(rn);
                                replacement = Some(Element::Number(0));
                                done = false;
                            }
                        }
                    }
                    if let Some(r) = replacement {
                        println!("replacing {} with {}", element, r);
                        *element = r;
                    } else if let Element::Pair(left, right) = element {
                        // have this after the replacement thing so the borrow checker
                        // doesn't think we're borrowing element when replacing it
                        stack.push((depth + 1, &mut *right));
                        stack.push((depth + 1, &mut *left));
                    } else if let Element::Number(number) = element {
                        if let Some(rn) = right_number {
                            println!("adding to right {}: {}", number, rn);
                            *number += rn;
                            break;
                        } else {
                            left_number = Some(number);
                        }
                    }
                }
            }

            // split loop
            {
                let mut stack = vec![(&mut *self.0), (&mut *self.1)];

                while let Some(element) = stack.pop() {
                    if let Element::Pair(left, right) = element {
                        stack.push(&mut *left);
                        stack.push(&mut *right);
                    } else if let Element::Number(number) = element {
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
            if done {
                break;
            }
        }
    }
}

pub fn run() -> std::io::Result<()> {
    // let data = std::fs::read_to_string("data/18.txt")?;

    let mut a = SnailNum::from("[[[[[9,8],1],2],3],4]");
    println!("{}", a);
    a.reduce();
    println!("{}", a);

    Ok(())
}
