use std::env;
use std::fs;
use std::io;
use hex::FromHex;
use std::cmp::min;
use std::cmp::max;

fn get_bit(bytes: &Vec<u8>, pos: &mut usize) -> u8 {

    let bit_pos = *pos % 8;
    let word = *pos / 8;

    *pos += 1;

    /* bit_pos : 01234567
     *    bits : 11010010
     *           ^      ^
     *           |      |
     *          MSB    LSB
     */

    (bytes[word] >> (7 - bit_pos)) & 0x01
}

fn parse_bits(bytes: &Vec<u8>, offset: &mut usize, n_bits: usize) -> u64 {

    let mut res = 0;

    for bit_pos in 0..n_bits {
        let bit = get_bit(bytes, offset);
        res |= (bit as u64) << (n_bits - bit_pos - 1);
    }

    res
}

fn sum_versions(packet: &Vec<u8>, offset: &mut usize) -> u64 {

    let version = parse_bits(packet, offset, 3);
    let type_id = parse_bits(packet, offset, 3);

    if type_id == 4 {
        loop {
            let bit = get_bit(packet, offset);
            *offset += 4;  /* skip 4-bits of literal */
            if bit != 1 { break; }
        }
        return version;
    }

    let mut sum = version;

    let length_type_id = get_bit(packet, offset);

    if length_type_id == 0 {
        let subpackets_len = parse_bits(packet, offset, 15) as usize;
        let final_offset = *offset + subpackets_len;
        while *offset < final_offset {
            sum += sum_versions(packet, offset);
        }
    } else {
        let n_subpackets = parse_bits(packet, offset, 11);
        for _ in 0..n_subpackets {
            sum += sum_versions(packet, offset);
        }
    }

    sum
}

fn get_new_value( packet: &Vec<u8>, offset: &mut usize,
                 type_id: u64,       value: u64) -> u64 {
    match type_id {
        0 => { return value + evaluate(packet, offset); },
        1 => { return value * evaluate(packet, offset); },
        2 => { return min(value, evaluate(packet, offset)); },
        3 => { return max(value, evaluate(packet, offset)); },
        5 | 6 | 7 => {
            let next_value = evaluate(packet, offset);
            return match type_id {
                5 => if value > next_value { 1 } else { 0 },
                6 => if value < next_value { 1 } else { 0 },
                7 => if value == next_value { 1 } else { 0 },
                _ => unreachable!(),
            }
        },
        _ => { unreachable!(); },
    }
}

fn evaluate(packet: &Vec<u8>, offset: &mut usize) -> u64 {

    /* Skip Version */
    *offset += 3;

    let type_id = parse_bits(packet, offset, 3);

    if type_id == 4 {
       let mut value = 0;
       loop {
           let bit = get_bit(packet, offset);
           value <<= 4;
           value |= parse_bits(packet, offset, 4);
           if bit != 1 { break; }
       }
       return value;
    }

    let length_type_id = get_bit(packet, offset);

    let mut value;

    if length_type_id == 0 {
        let subpackets_len = parse_bits(packet, offset, 15) as usize;
        let final_offset = *offset + subpackets_len;
        value = evaluate(packet, offset);
        while *offset < final_offset {
            value = get_new_value(packet, offset, type_id, value);
        }
    } else {  /* length_type_id == 1 */
        let n_subpackets = parse_bits(packet, offset, 11);
        value = evaluate(packet, offset);
        for _ in 0..(n_subpackets - 1) {
            value = get_new_value(packet, offset, type_id, value);
        }
    }

    value
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let hex_packet = if args.len() > 1 && args[1] != "-" {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Failed to read line");
        input
    };

    let packet = Vec::from_hex(hex_packet.trim())
        .expect("Invalid Hex Packet");

    let sum = sum_versions(&packet, &mut 0);
    println!("Answer to Part One : {}", sum);

    let value = evaluate(&packet, &mut 0);
    println!("Answer to Part Two : {}", value);
}
