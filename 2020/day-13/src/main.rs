use std::io::{self};

#[derive(Debug, Clone)]
struct BusTimestamp {
    bus_no: i32,
    timestamp: i32,
}

impl BusTimestamp {
    fn minutes_to_wait(&self, timestamp: i32) -> i32 {
        self.timestamp - timestamp
    }
}

fn main() -> io::Result<()> {
    let f = "test.txt";
    let f = "input.txt";

    let vec: Vec<String> = std::fs::read_to_string(f)?
        .lines()
        .map(|x| x.to_string())
        .collect();
    let timestamp = vec[0].parse::<i32>().unwrap();
    let bus_numbers: Vec<&str> = vec[1].split(",").collect();
    let mut first_bus = BusTimestamp{bus_no:-1, timestamp: timestamp * 10};
    for bus_number in bus_numbers.iter() {
        let number = match bus_number.parse::<i32>() {
            Ok(number) => number,
            Err(_e) => {
                continue
            },
        };
        let mut tmp = 0;
        while tmp <= timestamp {
            tmp += number;
        }
        if tmp < first_bus.timestamp {
            first_bus = BusTimestamp{bus_no: number, timestamp: tmp};
        }
    }
    println!("{:?}", first_bus);
    println!("{}", first_bus.minutes_to_wait(timestamp) * first_bus.bus_no);
    Ok(())
}
