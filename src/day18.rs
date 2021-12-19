use std::ops::Deref;

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
    fn add(&mut self, number: i32) {
        if let Element::Number(n) = self {
            *n += number;
        } else {
            panic!("not a number")
        }
    }
}

struct SnailNum(Box<Element>, Box<Element>);

impl SnailNum {
    fn new(input: &str) {}

    fn reduce(&mut self) {
        // parent loop
        // explode loop
        let mut stack = vec![(0, &mut *self.0), (0, &mut *self.1)];
        let mut left_number: Option<&mut i32> = None;
        let mut right_number: Option<i32> = None;

        while let Some((depth, element)) = stack.pop() {
            let mut replacement: Option<Element> = None;

            if let Element::Pair(left, right) = element {
                if depth > 4 {
                    let ln = left.number();
                    let rn = right.number();
                    if let Some(leftn) = left_number {
                        *leftn += ln;
                        left_number = None //this makes the borrow checker happy!
                    }
                    right_number = Some(rn);
                    replacement = Some(Element::Number(0));
                }
            }
            if let Some(r) = replacement {
                *element = r;
            } else if let Element::Pair(left, right) = element {
                // have this after the replacement thing so the borrow checker
                // doesn't think we're borrowing element when replacing it
                stack.push((depth + 1, &mut *left));
                stack.push((depth + 1, &mut *right));
            } else if let Element::Number(number) = element {
                left_number = Some(number);
            }
        }
        // split loop
    }
}

pub fn run() -> std::io::Result<()> {
    let data = std::fs::read_to_string("data/18.txt")?;

    Ok(())
}
