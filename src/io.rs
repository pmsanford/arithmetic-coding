use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct BitReader {
    cont: Vec<u8>,
    cur: u8,
    idx: u8,
    eof: bool,
}

impl BitReader {
    pub fn new(mut cont: Vec<u8>) -> Self {
        cont.reverse();
        let first = cont.pop().expect("No first byte");
        BitReader {
            cont,
            cur: first,
            idx: 7,
            eof: false,
        }
    }

    pub fn incr(&mut self) {
        if self.idx == 0 {
            if self.cont.len() == 0 {
                self.eof = true;
            } else {
                self.cur = self.cont.pop().expect("Ran out of bytes");
                self.idx = 7;
            }
        } else {
            self.idx -= 1;
        }
    }

    pub fn get_bit(&mut self) -> Option<u8> {
        if self.eof {
            return None;
        }
        let mask = 1 << self.idx;
        let result = Some(if self.cur & mask > 0 { 1 } else { 0 });
        self.incr();
        result
    }
}

#[derive(Debug)]
pub struct BitWriter {
    buf: Vec<u8>,
    cur: u8,
    idx: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        BitWriter {
            buf: Vec::new(),
            cur: 0,
            idx: 7,
        }
    }

    fn incr(&mut self) {
        if self.idx == 0 {
            self.buf.push(self.cur);
            self.cur = 0;
            self.idx = 7;
        } else {
            self.idx -= 1;
        }
    }
    pub fn put_bit(&mut self, bit: u8) {
        if bit > 0 {
            self.cur += 1 << self.idx;
        }
        self.incr();
    }
    pub fn get(mut self) -> Vec<u8> {
        if self.idx != 7 {
            self.buf.push(self.cur);
        }
        self.buf
    }
}

#[cfg(test)]
mod test {
    use super::BitReader;
    use super::BitWriter;

    #[test]
    fn reads_bits() {
        let mut rdr = BitReader::new(vec![0b1101_0011, 0b0010_0000]);
        let mut r1 = vec![];
        for _ in 0..8 {
            r1.push(rdr.get_bit().unwrap());
        }
        let mut r2 = vec![];
        for _ in 0..8 {
            r2.push(rdr.get_bit().unwrap());
        }

        assert_eq!(vec![1, 1, 0, 1, 0, 0, 1, 1], r1);
        assert_eq!(vec![0, 0, 1, 0, 0, 0, 0, 0], r2);
        assert_eq!(None, rdr.get_bit());
    }

    #[test]
    fn writes_bits() {
        let mut bw = BitWriter::new();
        let expected = 0b1101_0110;
        bw.put_bit(1);
        bw.put_bit(1);
        bw.put_bit(0);
        bw.put_bit(1);
        bw.put_bit(0);
        bw.put_bit(1);
        bw.put_bit(1);
        bw.put_bit(0);

        let res = bw.get();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], expected);
    }

    #[test]
    fn handles_partial() {
        let mut bw = BitWriter::new();
        let e1 = 0b0011_0110;
        let e2 = 0b0110_1000;

        bw.put_bit(0);
        bw.put_bit(0);
        bw.put_bit(1);
        bw.put_bit(1);
        bw.put_bit(0);
        bw.put_bit(1);
        bw.put_bit(1);
        bw.put_bit(0);

        bw.put_bit(0);
        bw.put_bit(1);
        bw.put_bit(1);
        bw.put_bit(0);
        bw.put_bit(1);

        let res = bw.get();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], e1);
        assert_eq!(res[1], e2);
    }
}
