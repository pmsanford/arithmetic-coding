use crate::common::*;
use crate::io::BitReader;
use crate::model::Model;
use anyhow::{Error, Result};
use std::fs::File;
use std::io::Read;

pub fn decompress(mut infile: File) -> Result<Vec<u8>> {
    let mut input = vec![];
    infile.read_to_end(&mut input)?;
    let mut input = BitReader::new(input);
    let mut high: u32 = MAX_CODE;
    let mut low: u32 = 0;
    let mut value: u32 = 0;
    let mut model = Model::new();
    let mut result = vec![];

    for _ in 0..CODE_VALUE_BITS {
        value <<= 1;
        value += input
            .get_bit()
            .ok_or_else(|| Error::msg(format!("File is shorter than {} bits", CODE_VALUE_BITS)))?
            as u32;
    }

    loop {
        let range = high - low + 1;
        let scaled_value = ((value - low + 1) * model.get_count() - 1) / range;
        let (c, p) = model.get_char(scaled_value)?;
        if c == 256 {
            break;
        }
        result.push(c as u8);

        high = low + (range * p.high) / p.count - 1;
        low = low + (range * p.low) / p.count;

        loop {
            if high < ONE_HALF {
            } else if low >= ONE_HALF {
                value -= ONE_HALF;
                low -= ONE_HALF;
                high -= ONE_HALF;
            } else if low >= ONE_FOURTH && high < THREE_FOURTHS {
                value -= ONE_FOURTH;
                low -= ONE_FOURTH;
                high -= ONE_FOURTH;
            } else {
                break;
            }

            low <<= 1;
            high <<= 1;
            high += 1;
            value <<= 1;
            value += input
                .get_bit()
                .ok_or_else(|| Error::msg("Ran out of bits!"))? as u32;
        }
    }

    Ok(result)
}
