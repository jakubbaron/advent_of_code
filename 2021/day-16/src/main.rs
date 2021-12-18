use std::io::{self};

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn string_to_binary(s: &str) -> String {
    s.chars().map(to_binary).collect()
}

#[derive(Debug)]
struct LiteralPacket {
    version: usize,
    packet_type_id: usize,
    value: usize,
    packet_length: usize,
}

fn str_to_usize(s: &str) -> usize {
    usize::from_str_radix(&s, 2).unwrap()
}

impl LiteralPacket {
    fn find_end(s: &str, start_idx: usize) -> usize {
        let mut idx = start_idx + 6;
        while &s[idx..idx + 1] == "1" {
            idx += 5;
        }
        idx + 5
    }

    fn get_str_groups(s: &str, start_idx: usize, end_idx: usize) -> String {
        let mut v: Vec<_> = vec![];
        let mut idx = start_idx;
        while idx != end_idx {
            v.push(&s[idx + 1..idx + 5]);
            idx += 5;
        }
        v.join("")
    }

    fn new(s: &str, start_idx: usize) -> LiteralPacket {
        let version = str_to_usize(&s[start_idx..start_idx + 3]);
        let packet_type_id = str_to_usize(&s[start_idx + 3..start_idx + 6]);
        let end_idx = LiteralPacket::find_end(s, start_idx);
        let value = str_to_usize(&LiteralPacket::get_str_groups(s, start_idx + 6, end_idx));
        let diff = (end_idx - (start_idx + 6)) % 5;
        let packet_length = (end_idx + diff) - start_idx;
        LiteralPacket {
            version,
            packet_type_id,
            value,
            packet_length,
        }
    }
}

#[derive(Debug)]
struct OperatorPacketBit15 {
    version: usize,
    packet_type_id: usize,
    length_type_id: usize,
    subpacket_length: usize,
    packet_length: usize,
}

impl OperatorPacketBit15 {
    fn new(s: &str, start_idx: usize) -> OperatorPacketBit15 {
        let mut idx = start_idx;
        let version = str_to_usize(&s[idx..idx + 3]);
        idx += 3;

        let packet_type_id = str_to_usize(&s[idx..idx + 3]);
        idx += 3;

        let length_type_id = str_to_usize(&s[idx..idx + 1]);
        idx += 1;

        let subpacket_length = str_to_usize(&s[idx..idx + 15]);
        idx += 15;
        OperatorPacketBit15 {
            version,
            packet_type_id,
            length_type_id,
            subpacket_length,
            packet_length: idx - start_idx,
        }
    }
}

#[derive(Debug)]
struct OperatorPacketBit11 {
    version: usize,
    packet_type_id: usize,
    length_type_id: usize,
    number_of_packets: usize,
    packet_length: usize,
}

impl OperatorPacketBit11 {
    fn new(s: &str, start_idx: usize) -> OperatorPacketBit11 {
        let mut idx = start_idx;
        let version = str_to_usize(&s[idx..idx + 3]);
        idx += 3;

        let packet_type_id = str_to_usize(&s[idx..idx + 3]);
        idx += 3;

        let length_type_id = str_to_usize(&s[idx..idx + 1]);
        idx += 1;

        let number_of_packets = str_to_usize(&s[idx..idx + 11]);
        idx += 11;
        OperatorPacketBit11 {
            version,
            packet_type_id,
            length_type_id,
            number_of_packets,
            packet_length: idx - start_idx,
        }
    }
}

#[derive(Debug)]
enum Packet {
    OperatorPacketBit11E(OperatorPacketBit11),
    OperatorPacketBit15E(OperatorPacketBit15),
    LiteralPacketE(LiteralPacket),
}

impl Packet {
    fn len(&self) -> usize {
        match *self {
            Packet::OperatorPacketBit15E(OperatorPacketBit15 { packet_length, .. }) => {
                packet_length
            }
            Packet::OperatorPacketBit11E(OperatorPacketBit11 { packet_length, .. }) => {
                packet_length
            }
            Packet::LiteralPacketE(LiteralPacket { packet_length, .. }) => packet_length,
        }
    }
    fn version(&self) -> usize {
        match *self {
            Packet::OperatorPacketBit15E(OperatorPacketBit15 { version, .. }) => version,
            Packet::OperatorPacketBit11E(OperatorPacketBit11 { version, .. }) => version,
            Packet::LiteralPacketE(LiteralPacket { version, .. }) => version,
        }
    }
}

fn get_packets(s: &str, packet_start: usize, end_idx: usize, packets: &mut Vec<Packet>) -> usize {
    let mut idx = packet_start;
    let version = str_to_usize(&s[idx..idx + 3]);
    idx += 3;
    let packet_type_id = str_to_usize(&s[idx..idx + 3]);
    idx += 3;
    match packet_type_id {
        4 => {
            println!("Constructing LiteralPacket");
            let packet = LiteralPacket::new(&s, packet_start);
            println!("{:?}", packet);
            let LiteralPacket { packet_length, .. } = packet;
            let new_idx = packet_start + packet_length;
            packets.push(Packet::LiteralPacketE(packet));
            return new_idx;
        }
        _ => {
            let length_type_id = str_to_usize(&s[idx..idx + 1]);
            idx += 1;
            match length_type_id {
                0 => {
                    println!("Constructing OperatorPacketBit15");
                    let packet = OperatorPacketBit15::new(&s, packet_start);
                    println!("{:?}", packet);
                    let OperatorPacketBit15 {
                        subpacket_length,
                        packet_length,
                        ..
                    } = packet;
                    packets.push(Packet::OperatorPacketBit15E(packet));
                    let mut new_idx = packet_start + packet_length;
                    let subpacket_end = new_idx + subpacket_length;
                    println!(
                        "new idx {}, idx + packet_length {}",
                        new_idx,
                        idx + packet_length
                    );
                    while new_idx < subpacket_end {
                        new_idx = get_packets(s, new_idx, end_idx, packets);
                    }
                    return new_idx;
                }
                1 => {
                    println!("Constructing OperatorPacketBit11");
                    let packet = OperatorPacketBit11::new(&s, packet_start);
                    println!("{:?}", packet);
                    let OperatorPacketBit11 {
                        number_of_packets,
                        packet_length,
                        ..
                    } = packet;
                    packets.push(Packet::OperatorPacketBit11E(packet));
                    let mut new_idx = packet_start + packet_length;
                    for _ in 0..number_of_packets {
                        new_idx = get_packets(&s, new_idx, end_idx, packets);
                    }
                    return new_idx;
                }

                _ => panic!("Nope!!"),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_4() {
        let data = "D2FE28";
        let s = string_to_binary(&data);
        let l = LiteralPacket::new(&s, 0);
        assert_eq!(l.value, 2021);
        assert_eq!(l.packet_length, 21);
    }

    #[test]
    fn operator_packet_bit15() {
        let data = "38006F45291200";
        let s = string_to_binary(&data);
        let l = OperatorPacketBit15::new(&s, 0);
        assert_eq!(l.subpacket_length, 27);
        assert_eq!(l.version, 1);
        assert_eq!(l.packet_type_id, 6);
        assert_eq!(l.length_type_id, 0);
        assert_eq!(l.packet_length, 22);

        let pp = LiteralPacket::new(&s, l.packet_length);
        assert_eq!(pp.version, 6);
        assert_eq!(pp.packet_type_id, 4);
        assert_eq!(pp.value, 10);
        assert_eq!(pp.packet_length, 11);
        let ppp = LiteralPacket::new(&s, l.packet_length + pp.packet_length);
        assert_eq!(ppp.version, 2);
        assert_eq!(ppp.packet_type_id, 4);
        assert_eq!(ppp.value, 20);
        assert_eq!(ppp.packet_length, 16);
    }

    #[test]
    fn operator_packet_bit11() {
        let data = "EE00D40C823060";
        let s = string_to_binary(&data);
        let l = OperatorPacketBit11::new(&s, 0);
        assert_eq!(l.version, 7);
        assert_eq!(l.packet_type_id, 3);
        assert_eq!(l.length_type_id, 1);
        assert_eq!(l.number_of_packets, 3);

        let p1 = LiteralPacket::new(&s, l.packet_length);
        assert_eq!(p1.value, 1);

        let p2 = LiteralPacket::new(&s, l.packet_length + p1.packet_length);
        assert_eq!(p2.value, 2);

        let p3 = LiteralPacket::new(&s, l.packet_length + p1.packet_length + p2.packet_length);
        assert_eq!(p3.value, 3);
    }

    #[test]
    fn first_example() {
        let data = "8A004A801A8002F478";
        let s = string_to_binary(&data);
        let mut packets: Vec<_> = vec![];
        println!("{}", s.len());
        let end = get_packets(&s, 0, s.len(), &mut packets);
        println!("{} {}", s.len(), end);
        println!("{:?}", packets);
        assert_eq!(packets.iter().map(Packet::version).sum::<usize>(), 16);
    }

    #[test]
    fn second_example() {
        let data = "620080001611562C8802118E34";
        let s = string_to_binary(&data);
        let mut packets: Vec<_> = vec![];
        println!("{}", s.len());
        let end = get_packets(&s, 0, s.len(), &mut packets);
        println!("{} {}", s.len(), end);
        println!("{:?}", packets);
        assert_eq!(packets.iter().map(Packet::version).sum::<usize>(), 12);
    }

    #[test]
    fn third_example() {
        let data = "C0015000016115A2E0802F182340";
        let s = string_to_binary(&data);
        let mut packets: Vec<_> = vec![];
        println!("{}", s.len());
        let end = get_packets(&s, 0, s.len(), &mut packets);
        println!("{} {}", s.len(), end);
        println!("{:?}", packets);
        assert_eq!(packets.iter().map(Packet::version).sum::<usize>(), 23);
    }

    #[test]
    fn fourth_example() {
        let data = "A0016C880162017C3686B18A3D4780";
        let s = string_to_binary(&data);
        let mut packets: Vec<_> = vec![];
        println!("{}", s.len());
        let end = get_packets(&s, 0, s.len(), &mut packets);
        println!("{} {}", s.len(), end);
        println!("{:?}", packets);
        assert_eq!(packets.iter().map(Packet::version).sum::<usize>(), 31);
    }
}

fn part_1(file_content: &Vec<String>) -> usize {
    let data_string = &file_content[0];
    let s = string_to_binary(&data_string);
    let mut packets: Vec<_> = vec![];

    let _ = get_packets(&s, 0, s.len(), &mut packets);
    packets.iter().map(Packet::version).sum::<usize>()
}

fn part_2(file_content: &Vec<String>) -> usize {
    0
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 16, 0),
        ("test1.txt", 12, 0),
        ("test2.txt", 23, 0),
        ("test3.txt", 31, 0),
        ("input.txt", 957, 0),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let res_1 = part_1(&file_content);
        assert_eq!(res_1, result_1);

        let res_2 = part_2(&file_content);
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
