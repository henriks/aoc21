macro_rules! ones {
    ($value:expr) => {
        (2u32.pow($value as u32) - 1) as u8
    };
}

struct Data(usize, Vec<u8>);

impl Data {
    fn new(data: Vec<u8>) -> Self {
        Data(0, data)
    }
    fn take(&mut self, bits: usize) -> u64 {
        let mut val = 0u64;
        let offset = self.0 % 8;
        let mut curr = self.1[(self.0 - offset) / 8] & ones!(8 - offset);
        let mut read = 8 - offset;
        while read < bits {
            val += curr as u64;
            curr = self.1[(self.0 + read) / 8];
            read += 8;
            val <<= 8;
        }
        self.0 += bits;
        (val + curr as u64) >> read - bits
    }
    fn reset(&mut self) {
        self.0 = 0;
    }
}

enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

fn read_packet(data: &mut Data) -> Packet {
    let version = data.take(3) as u8;
    let typeid = data.take(3) as u8;

    match typeid {
        4 => {
            let mut val = 0u64;
            loop {
                let bits = data.take(5);
                val <<= 4;
                val += bits & 0b1111;
                if bits & 1 << 5 != 0 {
                    break Packet::Literal(version, val);
                }
            }
        }
        _ => match data.take(1) {
            0 => {
                let len = data.take(15) as usize;
                let start = data.0;
                let mut packets = vec![];
                loop {
                    if len == data.0 - start {
                        break Packet::Operator(version, typeid, packets);
                    }
                    packets.push(read_packet(data));
                }
            }
            1 => {
                let len = data.take(11);
                let mut packets = vec![];
                for _ in 0..len {
                    packets.push(read_packet(data));
                }
                Packet::Operator(version, typeid, packets)
            }
            _ => panic!(),
        },
    }
}

pub fn run() -> std::io::Result<()> {
    let input = std::fs::read_to_string("data/16.txt")?;
    let mut data = Data::new(
        (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
            .collect::<Vec<_>>(),
    );

    let root = read_packet(&mut data);

    // println!("{:b}", data.take(5));
    // println!("{:b}", data.take(5));
    // println!("{:b}", data.take(5));
    // println!("{:b}", data.take(5));

    println!("puzzle 15.1 {}", "solve(&grid, 1)");

    Ok(())
}
