use prelude::v1::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitsError {
    OutOfBounds
}

pub trait BitsGet {
	fn get_bits_msb(&self, offset: usize, bits: usize) -> Result<u8, BitsError>;
	
    #[inline]
    fn get_bits_msb_slice(&self, offset: usize, bits: usize, output: &mut [u8]) -> Result<(), BitsError> {
        if bytes_required_for_bits(bits) > output.len() {
            return Err(BitsError::OutOfBounds);
        }

        let mut o = offset;
        let mut b = bits;
        for i in 0..output.len() {
            let l = min(8, b);
            output[i] = try!(self.get_bits_msb(o, l));
            o += l;
            b -= l;
        }
        Ok(())
    }    
}

pub trait BitsSet {
    fn set_bits_msb(&mut self, offset: usize, bits: usize, val: u8) -> Result<(), BitsError>;

    #[inline]
    fn set_bits_msb_slice(&mut self, offset: usize, bits: usize, val: &[u8]) -> Result<(), BitsError> {
        let mut o = offset;
        let mut b = bits;
        for &byte in val {
            let l = min(8, b);
            try!(self.set_bits_msb(o, l, byte));
            o += l;
            b -= l;
        } 

        Ok(())
    }
}


impl BitsGet for u8 {
    #[inline]
    fn get_bits_msb(&self, offset: usize, bits: usize) -> Result<u8, BitsError> {
        if offset >= 8 {
            return Err(BitsError::OutOfBounds);
        }
        if offset + bits > 8 {
            return Err(BitsError::OutOfBounds);
        }

        if bits == 0 { return Ok(0); }

		let o = 8-offset-bits;
		let mask = (ones(bits as u32) << o) as u8;
		Ok((self & mask) >> o)
    }
}

impl BitsSet for u8 {
    #[inline]
    fn set_bits_msb(&mut self, offset: usize, bits: usize, val: u8) -> Result<(), BitsError> {
        if offset >= 8 {
            return Err(BitsError::OutOfBounds);
        }
        if offset + bits > 8 {
            return Err(BitsError::OutOfBounds);
        }

        if bits == 0 { return Ok(()); }

        let o = 8-offset-bits;
		let b = ones(bits as u32);
		let mask = !((b << o) as u8);

		// zero the bits to be written
		let s = *self & mask;

		// mask the value
		let val = val & (b as u8);

		*self = s | ((val as u32) << o) as u8;

        Ok(())
    }
}

struct BitsRange { byte: usize, offset: usize, bits: usize }
fn split_ranges(bytes: usize, offset: usize, bits: usize) -> Result<(BitsRange, Option<BitsRange>), BitsError> {
    let total_bits = bytes * 8;

    if offset >= total_bits {
        return Err(BitsError::OutOfBounds);
    }
    if bits > 8 {
        return Err(BitsError::OutOfBounds);
    }
    if offset + bits > total_bits {
        return Err(BitsError::OutOfBounds);
    }

    let byte_start = offset / 8;
    let byte_end = (offset + bits - 1) / 8;
    
    if byte_start == byte_end {
        return Ok((BitsRange { 
            byte: byte_start,
            offset: offset % 8,
            bits: bits
        }, None));
    }

    let o = offset - (byte_start * 8);
    let b = (o + bits) % 8;

    let start = BitsRange {
        byte: byte_start,
        offset: o,
        bits: b
    };

    let o = (o + b) % 8;
    let b = bits - b;
    let end = BitsRange {
        byte: byte_end,
        offset: o,
        bits: b
    };

    Ok((start, Some(end)))    
}

pub fn bytes_required_for_bits(bits: usize) -> usize {
    let mut n = bits / 8;
    if (bits % 8) > 0 {
        n += 1;
    }
    n
}

impl<'a> BitsGet for &'a [u8] {
    #[inline]
    fn get_bits_msb(&self, offset: usize, bits: usize) -> Result<u8, BitsError> {
        if bits == 0 { return Ok(0); }
        
        let l = self.len();
        if l == 0 { return Err(BitsError::OutOfBounds); }
        
        match try!(split_ranges(l, offset, bits)) {
            (l, None) => {
                self[l.byte].get_bits_msb(l.offset, l.bits)
            },
            (l, Some(r)) => {
                let msb = try!(self[l.byte].get_bits_msb(l.offset, l.bits)) << l.bits;
                let lsb = try!(self[r.byte].get_bits_msb(r.offset, r.bits));
                Ok(msb | lsb)
            }
        }
    }
}


impl<'a> BitsSet for &'a mut [u8] {
    #[inline]
    fn set_bits_msb(&mut self, offset: usize, bits: usize, val: u8) -> Result<(), BitsError> {
        if bits == 0 { return Ok(()); }

        let l = self.len();
        if l == 0 { return Err(BitsError::OutOfBounds); }
        
        match try!(split_ranges(l, offset, bits)) {
            (l, None) => {
                try!(self[l.byte].set_bits_msb(l.offset, l.bits, val))
            },
            (l, Some(r)) => {
                let msb = val >> l.bits;
                try!(self[l.byte].set_bits_msb(l.offset, l.bits, msb));
                let lsb = val & (ones(r.bits as u32) as u8);
                try!(self[r.byte].set_bits_msb(r.offset, r.bits, lsb));
            }
        }

        Ok(())
    }
}



macro_rules! impl_bits_arr {
    ($T: ty) => (

        impl BitsGet for $T {
            #[inline]
            fn get_bits_msb(&self, offset: usize, bits: usize) -> Result<u8, BitsError> {
                if bits == 0 { return Ok(0); }
                
                let l = self.len();
                if l == 0 { return Err(BitsError::OutOfBounds); }
                
                match try!(split_ranges(l, offset, bits)) {
                    (l, None) => {
                        self[l.byte].get_bits_msb(l.offset, l.bits)
                    },
                    (l, Some(r)) => {
                        let msb = try!(self[l.byte].get_bits_msb(l.offset, l.bits)) << l.bits;
                        let lsb = try!(self[r.byte].get_bits_msb(r.offset, r.bits));
                        Ok(msb | lsb)
                    }
                }
            }
        }

        impl BitsSet for $T {
            #[inline]
            fn set_bits_msb(&mut self, offset: usize, bits: usize, val: u8) -> Result<(), BitsError> {
                if bits == 0 { return Ok(()); }

                let l = self.len();
                if l == 0 { return Err(BitsError::OutOfBounds); }
                
                match try!(split_ranges(l, offset, bits)) {
                    (l, None) => {
                        try!(self[l.byte].set_bits_msb(l.offset, l.bits, val))
                    },
                    (l, Some(r)) => {
                        let msb = val >> l.bits;
                        try!(self[l.byte].set_bits_msb(l.offset, l.bits, msb));
                        let lsb = val & (ones(r.bits as u32) as u8);
                        try!(self[r.byte].set_bits_msb(r.offset, r.bits, lsb));
                    }
                }

                Ok(())
            }
        }

    )
}


impl_bits_arr!([u8]);
impl_bits_arr!([u8; 1]);
impl_bits_arr!([u8; 2]);
impl_bits_arr!([u8; 3]);
impl_bits_arr!([u8; 4]);
impl_bits_arr!([u8; 5]);
impl_bits_arr!([u8; 6]);
impl_bits_arr!([u8; 7]);
impl_bits_arr!([u8; 8]);
impl_bits_arr!([u8; 9]);
impl_bits_arr!([u8; 10]);
impl_bits_arr!([u8; 11]);
impl_bits_arr!([u8; 12]);
impl_bits_arr!([u8; 13]);
impl_bits_arr!([u8; 14]);
impl_bits_arr!([u8; 15]);
impl_bits_arr!([u8; 16]);
impl_bits_arr!([u8; 17]);
impl_bits_arr!([u8; 18]);
impl_bits_arr!([u8; 19]);
impl_bits_arr!([u8; 20]);
impl_bits_arr!([u8; 21]);
impl_bits_arr!([u8; 22]);
impl_bits_arr!([u8; 23]);
impl_bits_arr!([u8; 24]);
impl_bits_arr!([u8; 25]);
impl_bits_arr!([u8; 26]);
impl_bits_arr!([u8; 27]);
impl_bits_arr!([u8; 28]);
impl_bits_arr!([u8; 29]);
impl_bits_arr!([u8; 30]);
impl_bits_arr!([u8; 31]);
impl_bits_arr!([u8; 32]);


pub fn ones(n: u32) -> u32 {
	if n == 0 { return 0; }
	if n >= 32 { return !0; }

	(1 << n) - 1
}

#[test]
#[cfg(test)]
fn test_u8() {
    {
		let n: u8 = 0b10000001;

		let m = n.get_bits_msb(0, 1).unwrap();
		assert_eq!(1, m);

		let m = n.get_bits_msb(7, 1).unwrap();
		assert_eq!(1, m);
	}
	
	{
		let n: u8 = 0b00110100;

		let m = n.get_bits_msb(0, 4).unwrap();
		assert_eq!(0b0011, m);

		let m = n.get_bits_msb(2, 4).unwrap();
		assert_eq!(0b1101, m);

		let m = n.get_bits_msb(4, 4).unwrap();
		assert_eq!(0b0100, m);

		let m = n.get_bits_msb(0, 8).unwrap();
		assert_eq!(n, m);

		let m = n.get_bits_msb(0, 0).unwrap();
		assert_eq!(0, m);
	}

	{
        let mut n: u8 = 0;
		n.set_bits_msb(0, 1, 1).unwrap();
		assert_eq!(1 << 7, n);

        let mut n: u8 = 0;
		n.set_bits_msb(7, 1, 1).unwrap();
		assert_eq!(1, n);

        let mut n: u8 = 0;
		n.set_bits_msb(0, 8, 124).unwrap();
		assert_eq!(124, n);
	}

	{
		let mut n: u8 = 0b10101010;

		n.set_bits_msb(1, 1, 1).unwrap();
		assert_eq!(0b11101010, n);
	}    
}

#[test]
#[cfg(test)]
fn test_u8_slice_read() {
    let n = [0b00010001, 0b11001100];
    assert_eq!(17, n[0]);
    assert_eq!(204, n[1]);

    assert_eq!(17, n.get_bits_msb(0, 8).unwrap());
    assert_eq!(204, n.get_bits_msb(8, 8).unwrap());

    
    assert_eq!(0b11, n.get_bits_msb(7, 2).unwrap());    
    assert_eq!(0b0111, n.get_bits_msb(6, 4).unwrap());

    assert_eq!(0b00011100, n.get_bits_msb(4, 8).unwrap());

    assert_eq!(0b1, n.get_bits_msb(7, 1).unwrap());
    assert_eq!(0b1, n.get_bits_msb(8, 1).unwrap());
    assert_eq!(0b0, n.get_bits_msb(15, 1).unwrap());
}

#[test]
#[cfg(test)]
fn test_u8_slice_write() {
    {
        let mut n = [0, 0];
        n.set_bits_msb(6, 4, 0b1001).unwrap();
        assert_eq!(&[0b00000010, 0b01000000], &n);
    }

    {
        let mut n = [0b01000000, 0b00000010];
        n.set_bits_msb(6, 4, 0b1001).unwrap();
        assert_eq!(&[0b01000010, 0b01000010], &n);
    }
}

#[test]
#[cfg(test)]
fn test_large_write() {
    let mut n = [128; 32];
    n.set_bits_msb(0, 8, 255).unwrap();
    n.set_bits_msb(64 + 1, 1, 1).unwrap();
    n.set_bits_msb(72, 8, 133).unwrap();
    n.set_bits_msb((31 * 8) + 7, 1, 1).unwrap(); 
    assert_eq!(n[0], 255);    
    assert_eq!(n[8], 128 + 64);
    assert_eq!(n[9], 133);
    assert_eq!(n[31], 129);    
}

#[test]
#[cfg(test)]
fn test_slice_write() {
    {
        let mut n = [0, 0];
        n.set_bits_msb_slice(0, 16, &[0b10101010, 0b01010101]).unwrap();
        assert_eq!(&n, &[0b10101010, 0b01010101]);
    }

    {
        let mut n = [0; 4];
        n.set_bits_msb_slice(0, 32, &[255; 4]).unwrap();
        assert_eq!(&n, &[255; 4]);
    }
}

#[test]
#[cfg(test)]
fn test_slice_read() {
    {
        let n = [0b00001111, 0b11110000];
        let mut m = [0; 2];
        n.get_bits_msb_slice(0, 16, &mut m).unwrap();
        assert_eq!(&n, &m);
    }
}

#[test]
#[cfg(test)]
fn test_bits_slice() {
    let n = [128; 4];
    let s: &[u8] = &n;
    assert_eq!(128, s.get_bits_msb(0, 8).unwrap()); 
}

#[cfg(test)]
#[test]
fn test_ones() {
	for i in 0..33 {
		let n = ones(i);
		assert_eq!(i, n.count_ones());
	}
}
