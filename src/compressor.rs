use crate::io::BitWriter;
use crate::model::Model;
use anyhow::Result;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{BufReader, Read};

const MAX_CODE: u32 = 0xFFFF;
const ONE_FOURTH: u32 = 0x4000;
const ONE_HALF: u32 = 0x8000;
const THREE_FOURTHS: u32 = 0xC000;

fn get_byte(reader: &mut BufReader<File>) -> Result<u32> {
    let mut buf = [0u8; 1];
    match reader.read_exact(&mut buf) {
        Ok(()) => Ok(buf[0] as u32),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(256),
        Err(e) => Err(e)?,
    }
}

pub fn compress(input: File) -> Result<Vec<u8>> {
    let mut pending_bits = 0;
    let mut low: u32 = 0;
    let mut high: u32 = MAX_CODE;
    let mut reader = BufReader::new(input);
    let mut writer = BitWriter::new();
    let mut model = Model::new();

    loop {
        let c = get_byte(&mut reader)?;

        let charstring = if c > 0x20 && c <= 0x7F {
            format!("({})", c as u8 as char)
        } else {
            String::from("")
        };
        print!("0x{:02X}{} 0x{:02X} 0x{:02X} => ", c, charstring, low, high);

        let p = model.get_probability(c);
        let range = high - low + 1;
        high = low + (range * p.high / p.count) - 1;
        low = low + (range * p.low / p.count);

        println!("0x{:02X} 0x{:02X}", low, high);

        loop {
            if high < ONE_HALF {
                put_bit_plus_pending(&mut writer, 0, &mut pending_bits);
            } else if low >= ONE_HALF {
                put_bit_plus_pending(&mut writer, 1, &mut pending_bits);
            } else if low >= ONE_FOURTH && high < THREE_FOURTHS {
                pending_bits += 1;
                low -= ONE_FOURTH;
                high -= ONE_FOURTH;
            } else {
                break;
            }

            high <<= 1;
            high += 1;
            low <<= 1;
            high &= MAX_CODE;
            low &= MAX_CODE;
        }
        if c == 256 {
            break;
        }
    }
    pending_bits += 1;
    if low < ONE_FOURTH {
        put_bit_plus_pending(&mut writer, 0, &mut pending_bits)
    } else {
        put_bit_plus_pending(&mut writer, 1, &mut pending_bits)
    }

    Ok(writer.get())
}

fn put_bit_plus_pending(wr: &mut BitWriter, bit: u8, pending_bits: &mut u32) {
    wr.put_bit(bit);
    let excess = if bit == 0 { 1 } else { 0 };
    for _ in 0..*pending_bits {
        wr.put_bit(excess);
    }

    *pending_bits = 0;
}
