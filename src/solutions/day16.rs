#[derive(Clone, Copy, Default, Debug)]
struct Packet {
    version: usize,
    length: usize,
    value: usize,
}

fn bits_to_val(bits: &[u8]) -> usize {
    let mut val: usize = 0;
    for b in bits {
        val <<= 1;
        val += (b & 1) as usize;
    }
    val
}

fn consume_bits<'a>(bits: &'a [u8], index: &mut usize, num_bits: usize) -> &'a [u8] {
    let part = &bits[*index..(*index + num_bits)];
    *index += num_bits;
    part
}

fn parse_literal(bits: &[u8], packet: &mut Packet) {
    loop {
        let terminate = consume_bits(bits, &mut packet.length, 1)[0] == 0;
        packet.value <<= 4;
        packet.value += bits_to_val(consume_bits(bits, &mut packet.length, 4));
        if terminate {
            break;
        }
    }
}

fn parse_operator(bits: &[u8], packet: &mut Packet, id: u8, packet_type: usize) {
    let mut sub_packets = Vec::new();
    if id == 0 {
        let subpackets_length = bits_to_val(consume_bits(bits, &mut packet.length, 15));
        let mut length_iterated_over = 0;
        while length_iterated_over < subpackets_length {
            let sub_packet = parse_packet(&bits[packet.length..]);
            length_iterated_over += sub_packet.length;
            packet.length += sub_packet.length;
            sub_packets.push(sub_packet);
        }
    } else {
        let subpackets_count = bits_to_val(consume_bits(bits, &mut packet.length, 11));
        for _ in 0..subpackets_count {
            let sub_packet = parse_packet(&bits[packet.length..]);
            packet.length += sub_packet.length;
            sub_packets.push(sub_packet);
        }
    }

    let mut iter = sub_packets.iter().map(|p| p.value);
    packet.value = match packet_type {
        0 => iter.sum(),
        1 => iter.product(),
        2 => iter.min().unwrap(),
        3 => iter.max().unwrap(),
        5 => (iter.next().unwrap() > iter.next().unwrap()) as usize,
        6 => (iter.next().unwrap() < iter.next().unwrap()) as usize,
        7 => (iter.next().unwrap() == iter.next().unwrap()) as usize,
        _ => {
            println!("Shouldn't get here!");
            0
        }
    };

    packet.version += sub_packets.iter().map(|p| p.version).sum::<usize>();
}

fn parse_packet(bits: &[u8]) -> Packet {
    let mut packet = Packet::default();
    packet.version = bits_to_val(consume_bits(bits, &mut packet.length, 3));

    let ptype = bits_to_val(consume_bits(bits, &mut packet.length, 3));
    match ptype {
        4 => {
            parse_literal(bits, &mut packet);
        }
        _ => {
            let id = consume_bits(bits, &mut packet.length, 1)[0];
            parse_operator(bits, &mut packet, id, ptype);
        }
    };
    packet
}

fn part1(bits: &Vec<u8>) -> usize {
    let packet = parse_packet(&bits[..]);
    packet.version
}

fn part2(bits: &Vec<u8>) -> usize {
    let packet = parse_packet(&bits[..]);
    packet.value
}

pub fn solve(input: String) {
    let mut bits = Vec::new();
    for c in input.chars() {
        if let Some(n) = c.to_digit(16) {
            for i in (0..4).rev() {
                bits.push((n as u8 >> i) & 1);
            }
        }
    }

    println!("Part 1: {}", part1(&bits));
    println!("Part 1: {}", part2(&bits));
}
