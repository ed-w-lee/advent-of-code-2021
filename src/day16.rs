#[derive(Debug)]
struct Literal {
    version: u32,
    value: u64,
}

#[derive(Debug)]
struct Operator {
    version: u32,
    packet_type: u32,
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet {
    Op(Operator),
    Lit(Literal),
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

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
        _ => unreachable!(),
    }
}

fn parse_packet(binary: &str) -> (Packet, usize) {
    let version = u32::from_str_radix(&binary[0..3], 2).unwrap();
    let packet_type = u32::from_str_radix(&binary[3..6], 2).unwrap();
    if packet_type == 4 {
        // literal
        let mut n_chunk = 0;
        let mut val: u64 = 0;
        loop {
            val <<= 4;
            // take 5 at a time, once we hit 0
            n_chunk += 1;
            let start_idx = (n_chunk * 5) + 1;
            let end_idx = ((n_chunk + 1) * 5) + 1;
            let chunk = &binary[start_idx..end_idx];

            let chunk_val = u64::from_str_radix(&chunk[1..], 2).unwrap();
            val += chunk_val;
            if chunk.starts_with('0') {
                break;
            }
        }
        // let n_bits_parsed = ((n_chunk * 5 + 6 + 3) / 4) * 4;
        let n_bits_parsed = n_chunk * 5 + 6;
        (
            Packet::Lit(Literal {
                version,
                value: val,
            }),
            n_bits_parsed,
        )
    } else {
        // operator
        if binary.chars().nth(6) == Some('0') {
            let bit_length = usize::from_str_radix(&binary[7..22], 2).unwrap();
            let subpackets_str = &binary[22..(22 + bit_length)];
            let mut n_idx = 0;
            let mut subpackets: Vec<Packet> = Vec::new();
            while n_idx < subpackets_str.len() {
                let (subpacket, num_bits) = parse_packet(&subpackets_str[n_idx..]);
                subpackets.push(subpacket);
                n_idx += num_bits;
            }
            (
                Packet::Op(Operator {
                    version,
                    packet_type,
                    subpackets,
                }),
                22 + bit_length,
            )
        } else {
            let num_subpackets = usize::from_str_radix(&binary[7..18], 2).unwrap();
            let mut subpackets: Vec<Packet> = Vec::new();
            let mut next_idx = 18;
            for _ in 0..num_subpackets {
                let (subpacket, num_bits) = parse_packet(&binary[next_idx..]);
                subpackets.push(subpacket);
                next_idx += num_bits;
            }
            (
                Packet::Op(Operator {
                    version,
                    packet_type,
                    subpackets,
                }),
                next_idx,
            )
        }
    }
}

fn sum_versions(packet: Packet) -> u32 {
    match packet {
        Packet::Op(Operator {
            version,
            packet_type: _,
            subpackets,
        }) => {
            let sub_sums: u32 = subpackets.into_iter().map(sum_versions).sum();
            sub_sums + version
        }
        Packet::Lit(Literal { version, value: _ }) => version,
    }
}

fn evaluate(packet: Packet) -> u64 {
    match packet {
        Packet::Op(Operator {
            version: _,
            packet_type,
            subpackets,
        }) => match packet_type {
            0 => subpackets.into_iter().map(evaluate).sum(),
            1 => subpackets.into_iter().map(evaluate).product(),
            2 => subpackets.into_iter().map(evaluate).min().unwrap(),
            3 => subpackets.into_iter().map(evaluate).max().unwrap(),
            5 => {
                let evaluated: Vec<_> = subpackets.into_iter().map(evaluate).collect();
                if evaluated[0] > evaluated[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                let evaluated: Vec<_> = subpackets.into_iter().map(evaluate).collect();
                if evaluated[0] < evaluated[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                let evaluated: Vec<_> = subpackets.into_iter().map(evaluate).collect();
                if evaluated[0] == evaluated[1] {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        },
        Packet::Lit(Literal { version: _, value }) => value,
    }
}

pub fn solution_1(line: &str) -> u32 {
    let binary = convert_to_binary_from_hex(line);
    let packet = parse_packet(&binary).0;
    sum_versions(packet)
}

pub fn solution_2(line: &str) -> u64 {
    let binary = convert_to_binary_from_hex(line);
    let packet = parse_packet(&binary).0;
    evaluate(packet)
}
