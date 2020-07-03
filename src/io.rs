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
    use super::BitWriter;

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
