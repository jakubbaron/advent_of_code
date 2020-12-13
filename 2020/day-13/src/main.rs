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
    fn minutes_to_wait(&self, timestamp: i32) -> i32 {
        self.timestamp - timestamp
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 1068781_u64),
        ("test2.txt", 3417_u64),
        ("test3.txt", 754018_u64),
        ("test4.txt", 779210_u64),
        ("test5.txt", 1261476_u64),
        ("test6.txt", 1202161486_u64),
        ("input.txt", 1_u64),
    ];
    for (f, result) in files_results.iter() {
        println!("{}", f);
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

        let mut bus_offsets: Vec<BusOffset> = Vec::new();
        for (offset, bus_no) in bus_numbers.iter().enumerate() {
            let number = match bus_no.parse::<u64>() {
                Ok(number) => number,
                Err(_e) => {
                    continue
                },
            };
            println!("{} {}", number, offset);
            bus_offsets.push(BusOffset{ bus_no: number, offset: offset as u64 });
        }

        let BusOffset{bus_no: last_bus_no, offset: last_offset}= bus_offsets.last().unwrap();
        let BusOffset{bus_no: first_bus_no, offset: _}= bus_offsets.first().unwrap();
        let end_range = last_bus_no * (first_bus_no + last_offset);
        let mut start_range = *first_bus_no;
        let range_offset = first_bus_no;
        let mut all_match = false;
        println!("{} {} {}", start_range, end_range, range_offset);

        while !all_match {
            all_match = true;
            for bus_offset in bus_offsets.iter() {
                let BusOffset{bus_no, offset} = bus_offset;
                if (start_range + offset) % bus_no != 0 {
                    all_match = false;
                    break;
                }
            }
            if !all_match {
                start_range += range_offset;
            }
        }
        println!("{}", start_range);
        assert_eq!(start_range, *result);
    }
    Ok(())
}
