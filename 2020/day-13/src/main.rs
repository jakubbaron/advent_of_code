use std::io::{self};

#[derive(Debug, Clone)]
struct BusTimestamp {
    bus_no: i32,
    timestamp: i32,
}

struct BusOffset {
    bus_no: u64,
    offset: u64,
}

impl BusTimestamp {
    fn part_1(&self, timestamp: i32) -> i32 {
        (self.timestamp - timestamp) * self.bus_no
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 295, 1068781_u64),
        ("test2.txt", 130, 3417_u64),
        ("test3.txt", 295, 754018_u64),
        ("test4.txt", 295, 779210_u64),
        ("test5.txt", 295, 1261476_u64),
        ("test6.txt", 47, 1202161486_u64),
        ("input.txt", 5946, 645338524823718_u64),
        ("input2.txt", 2845, 487905974205117_u64),
    ];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let vec: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let timestamp = vec[0].parse::<i32>().unwrap();
        let bus_numbers: Vec<&str> = vec[1].split(",").collect();
        let mut first_bus = BusTimestamp {
            bus_no: -1,
            timestamp: timestamp * 10,
        };
        for bus_number in bus_numbers.iter() {
            let number = match bus_number.parse::<i32>() {
                Ok(number) => number,
                Err(_e) => continue,
            };
            let mut tmp = 0;
            while tmp <= timestamp {
                tmp += number;
            }
            if tmp < first_bus.timestamp {
                first_bus = BusTimestamp {
                    bus_no: number,
                    timestamp: tmp,
                };
            }
        }
        println!("Minutes to wait {}", first_bus.part_1(timestamp));
        assert_eq!(first_bus.part_1(timestamp), *result_1);

        // Part Two
        let mut bus_offsets: Vec<BusOffset> = Vec::new();
        for (offset, bus_no) in bus_numbers.iter().enumerate() {
            let number = match bus_no.parse::<u64>() {
                Ok(number) => number,
                Err(_e) => continue,
            };
            bus_offsets.push(BusOffset {
                bus_no: number,
                offset: offset as u64,
            });
        }

        let (first_bus, other_buses) = bus_offsets.split_first().unwrap();
        let mut interval = first_bus.bus_no;
        let mut start = 0_u64;
        for curr_bus in other_buses.iter() {
            let BusOffset { bus_no, offset } = curr_bus;
            // find first occurence
            while (start + offset) % bus_no != 0 {
                start += interval;
            }
            // now buses are going to be aligned at the interval of below
            interval *= bus_no;
        }
        println!("First common occurence: {}", start);
        assert_eq!(start, *result_2);
    }
    Ok(())
}
