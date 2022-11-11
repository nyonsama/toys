use bits::Bits;
use des_constants::*;

// 2022.11.11后记
// 从维基百科复制下来的SBOX顺序不太合适，直接用的话会在运行时多一点开销
// 所以可以用const来在编译时给SBOX重新排一下序
// 但是当时还不知道const的这个功能
// 结果用了宏来解决这个问题
// 绕远了

#[cfg(test)]
mod des_tests {

    #[test]
    fn print_const_array() {
        println!("{:?}", super::des_constants::IP);
    }

    #[test]
    fn test_sbox_macro() {
        use super::des_constants::*;
        // let _sbox: Vec<u16> = (0..512).collect();
        let sbox = SBOX_UNSORTED;
        // let mut fake_sbox_sorted = [0u16; 512];
        let mut sbox_sorted = SBOX_UNSORTED;

        for box_ in 0..8 {
            for j in 0..64 {
                let row = ((j & 1) << 1) | (j >> 5);
                let col = ((((j & 0b011110) >> 1) as u64).reverse_bits() >> 60) as usize;

                sbox_sorted[box_ * 64 + j] = sbox[box_ * 64 + row * 16 + col];
            }
        }
        assert_eq!(SBOX, sbox_sorted);

        const SBOX_UNSORTED: [u8; 512] = [
            14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7, 0, 15, 7, 4, 14, 2, 13, 1, 10, 6,
            12, 11, 9, 5, 3, 8, 4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0, 15, 12, 8, 2,
            4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13, 15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0,
            5, 10, 3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5, 0, 14, 7, 11, 10, 4, 13,
            1, 5, 8, 12, 6, 9, 3, 2, 15, 13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9, 10,
            0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8, 13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5,
            14, 12, 11, 15, 1, 13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7, 1, 10, 13, 0,
            6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12, 7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12,
            4, 15, 13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9, 10, 6, 9, 0, 12, 11, 7,
            13, 15, 1, 3, 14, 5, 2, 8, 4, 3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14, 2,
            12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9, 14, 11, 2, 12, 4, 7, 13, 1, 5, 0,
            15, 10, 3, 9, 8, 6, 4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14, 11, 8, 12, 7,
            1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3, 12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7,
            5, 11, 10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8, 9, 14, 15, 5, 2, 8, 12, 3,
            7, 0, 4, 10, 1, 13, 11, 6, 4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13, 4, 11,
            2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1, 13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12,
            2, 15, 8, 6, 1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2, 6, 11, 13, 8, 1, 4,
            10, 7, 9, 5, 0, 15, 14, 2, 3, 12, 13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7,
            1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2, 7, 11, 4, 1, 9, 12, 14, 2, 0, 6,
            10, 13, 15, 3, 5, 8, 2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11,
        ];
    }

    #[test]
    fn test_encode_decode() {
        use super::*;
        let data = u8x8_to_u64(b"asdfghjk");
        // let data = 0b1101011001010110000101101110011001100110001001101100111010000110;
        let key = u8x8_to_u64(b"qwertyui");
        // let key = 0b1001011010101110100111100010111001001110101001101110111010001110;
        let cipher = encrypt_u64(data, key);
        let plain = decrypt_u64(cipher, key);

        println!("{}", data);
        println!("{}", cipher);
        println!("{}", plain);

        use std::io::Write;
        std::io::stdout()
            .write(u64_to_u8x8(plain).as_slice())
            .unwrap();
        println!();

        assert_eq!(data, plain);
    }

    #[test]
    fn test_enc_dec_bytes() {
        use super::*;
        let mut data = b"can i help you find some thing?".to_vec();
        if data.len() % 8 != 0 {
            data.resize((data.len() / 8 + 1) * 8, 0);
        }
        let key = b"holycrap";
        let cipher = encrypt_bytes(data.as_slice(), key);
        let plain = decrypt_bytes(cipher.as_slice(), key);
        assert_eq!(data.as_slice(), plain.as_slice());
    }
}

pub fn encrypt_bytes(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut u64s: Vec<u64> = Vec::new();

    // let mut i = bytes.iter();
    // loop {
    //     let mut data = 0u64;
    //     for shift in 0..8 {
    //         let b = match i.next() {
    //             Some(byte) => byte.clone() as u64,
    //             None => break,
    //         };
    //         data |= b << shift * 8;
    //     }
    //     u64s.push(data);
    //     if i.len() == 0 {
    //         break;
    //     }
    // }

    let mut i = bytes.chunks(8);
    loop {
        match i.next() {
            None => break,
            Some(bs) => u64s.push(u8x8_to_u64(bs)),
        }
    }

    let mut ret: Vec<u8> = Vec::new();
    for plaindata in u64s {
        // ret.push(encode_u64(plaindata, u8x8_to_u64(key)));
        ret.append(u64_to_u8x8(encrypt_u64(plaindata, u8x8_to_u64(key))).as_mut());
    }
    ret
}

pub fn decrypt_bytes(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut u64s: Vec<u64> = Vec::new();

    let mut i = bytes.chunks(8);
    loop {
        match i.next() {
            None => break,
            Some(bs) => u64s.push(u8x8_to_u64(bs)),
        }
    }

    let mut ret: Vec<u8> = Vec::new();
    for cipherdata in u64s {
        ret.append(u64_to_u8x8(decrypt_u64(cipherdata, u8x8_to_u64(key))).as_mut());
    }
    ret
}

fn u8x8_to_u64(data: &[u8]) -> u64 {
    assert_eq!(data.len() <= 8, true);

    let mut ret = 0u64;
    let mut i = data.iter().rev();
    loop {
        match i.next() {
            None => break,
            Some(byte) => {
                ret <<= 8;
                ret |= byte.clone() as u64;
            }
        }
    }
    ret
}

fn u64_to_u8x8(data: u64) -> Vec<u8> {
    let mut data = data;
    let mut ret: Vec<u8> = Vec::new();
    for _ in 0..8 {
        let value = data & !(u64::MAX << 8);
        data >>= 8;
        ret.push(value as u8)
    }
    ret
}

//返回密文
pub fn encrypt_u64(plaindata: u64, key: u64) -> u64 {
    worker(plaindata, &keygen(key)).to_u64()
}

pub fn decrypt_u64(cipherdata: u64, key: u64) -> u64 {
    let mut sub_keys = keygen(key);
    sub_keys.reverse();
    worker(cipherdata, &sub_keys).to_u64()
}

fn keygen(key: u64) -> Vec<Bits> {
    let mut key56_pc1 = Bits::from_u64(key, 64).get_bits(&PC1);
    let mut key_l = key56_pc1.popn_le(28);
    let mut key_r = key56_pc1;
    let mut sub_keys = Vec::new();

    //pc2
    for i in 0..16 {
        key_r.shl_loop(KEY_SHIFTS[i] as usize);
        key_l.shl_loop(KEY_SHIFTS[i] as usize);

        let mut key48_shl = key_l.clone();
        key48_shl.pushn_he(&key_r);
        sub_keys.push(key48_shl.get_bits(&PC2));
    }
    sub_keys
}

fn worker(data: u64, sub_keys: &Vec<Bits>) -> Bits {
    //ip
    let mut ip = Bits::new(data, 64).get_bits(&IP);

    let mut half_l = ip.popn_le(32);
    let mut half_r = ip;

    //16次循环
    for i in 0..16 {
        let temp = half_r.clone();
        half_r = half_l.bitxor(&f_func(&half_r, &sub_keys[i]));
        half_l = temp;
    }
    half_r.pushn_he(&half_l);

    //反向ip
    half_r.get_bits(&FP)
}

fn f_func(half_block: &Bits, sub_key: &Bits) -> Bits {
    let mut after_xor = half_block.get_bits(&EBOX).bitxor(sub_key);

    let mut ret = Bits::new(0, 0);
    //过s盒
    for _box in 0..8 {
        ret.pushn_he(&Bits::from_u64(
            SBOX[after_xor.popn_le(6).to_u64() as usize] as u64,
            4,
        ));

        // // calcute the index of sbox at runtime
        // let bitx6 = after_xor.popn_le(6).to_u64();
        // let row = ((bitx6 & 1) << 1) | (bitx6 >> 5);
        // let col = (((bitx6 & 0b011110) >> 1) as u64).reverse_bits() >> 60;
        // let index = _box * 64 + row * 16 + col;
        // ret.pushn_he(&Bits::from_u64(SBOX_UNSORTED[index as usize] as u64, 4));
    }

    ret.get_bits(&PBOX)
}

//把数组字面值的每一个参数减1
macro_rules! array_all_dec_1 {
    ($($x:expr),*) => [
        [ $(($x) - 1, )* ]
    ];
}

//给sbox排序，实现把6bit丢进数组索引直接得到4bit
macro_rules! sort_the_fucking_sbox {

    //第一层递归
    ([
        $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr,
        $v8:expr, $v9:expr, $v10:expr, $v11:expr, $v12:expr, $v13:expr, $v14:expr, $v15:expr,
        $v16:expr, $v17:expr, $v18:expr, $v19:expr, $v20:expr, $v21:expr, $v22:expr, $v23:expr,
        $v24:expr, $v25:expr, $v26:expr, $v27:expr, $v28:expr, $v29:expr, $v30:expr, $v31:expr,
        $v32:expr, $v33:expr, $v34:expr, $v35:expr, $v36:expr, $v37:expr, $v38:expr, $v39:expr,
        $v40:expr, $v41:expr, $v42:expr, $v43:expr, $v44:expr, $v45:expr, $v46:expr, $v47:expr,
        $v48:expr, $v49:expr, $v50:expr, $v51:expr, $v52:expr, $v53:expr, $v54:expr, $v55:expr,
        $v56:expr, $v57:expr, $v58:expr, $v59:expr, $v60:expr, $v61:expr, $v62:expr, $v63:expr,
        $($left:expr),*$(,)*
    ]) => {
        sort_the_fucking_sbox!([
            $v0, $v32, $v8, $v40, $v4, $v36, $v12, $v44, $v2, $v34, $v10, $v42, $v6, $v38, $v14,
            $v46, $v1, $v33, $v9, $v41, $v5, $v37, $v13, $v45, $v3, $v35, $v11, $v43, $v7, $v39,
            $v15, $v47, $v16, $v48, $v24, $v56, $v20, $v52, $v28, $v60, $v18, $v50, $v26, $v58,
            $v22, $v54, $v30, $v62, $v17, $v49, $v25, $v57, $v21, $v53, $v29, $v61, $v19, $v51,
            $v27, $v59, $v23, $v55, $v31, $v63,
            ],
            $($left),*
        )
    };

    //中间的递归
    (
        [$($sorted:expr),*$(,)*],
        $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr,
        $v8:expr, $v9:expr, $v10:expr, $v11:expr, $v12:expr, $v13:expr, $v14:expr, $v15:expr,
        $v16:expr, $v17:expr, $v18:expr, $v19:expr, $v20:expr, $v21:expr, $v22:expr, $v23:expr,
        $v24:expr, $v25:expr, $v26:expr, $v27:expr, $v28:expr, $v29:expr, $v30:expr, $v31:expr,
        $v32:expr, $v33:expr, $v34:expr, $v35:expr, $v36:expr, $v37:expr, $v38:expr, $v39:expr,
        $v40:expr, $v41:expr, $v42:expr, $v43:expr, $v44:expr, $v45:expr, $v46:expr, $v47:expr,
        $v48:expr, $v49:expr, $v50:expr, $v51:expr, $v52:expr, $v53:expr, $v54:expr, $v55:expr,
        $v56:expr, $v57:expr, $v58:expr, $v59:expr, $v60:expr, $v61:expr, $v62:expr, $v63:expr,
        $($left:expr),*$(,)*
    ) => {
        sort_the_fucking_sbox!([
            $($sorted),*,
            $v0, $v32, $v8, $v40, $v4, $v36, $v12, $v44, $v2, $v34, $v10, $v42, $v6, $v38, $v14,
            $v46, $v1, $v33, $v9, $v41, $v5, $v37, $v13, $v45, $v3, $v35, $v11, $v43, $v7, $v39,
            $v15, $v47, $v16, $v48, $v24, $v56, $v20, $v52, $v28, $v60, $v18, $v50, $v26, $v58,
            $v22, $v54, $v30, $v62, $v17, $v49, $v25, $v57, $v21, $v53, $v29, $v61, $v19, $v51,
            $v27, $v59, $v23, $v55, $v31, $v63,
            ],
            $($left),*
        )
    };

    //最深一层递归
    (
        [$($sorted:expr),*$(,)*],
        $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr,
        $v8:expr, $v9:expr, $v10:expr, $v11:expr, $v12:expr, $v13:expr, $v14:expr, $v15:expr,
        $v16:expr, $v17:expr, $v18:expr, $v19:expr, $v20:expr, $v21:expr, $v22:expr, $v23:expr,
        $v24:expr, $v25:expr, $v26:expr, $v27:expr, $v28:expr, $v29:expr, $v30:expr, $v31:expr,
        $v32:expr, $v33:expr, $v34:expr, $v35:expr, $v36:expr, $v37:expr, $v38:expr, $v39:expr,
        $v40:expr, $v41:expr, $v42:expr, $v43:expr, $v44:expr, $v45:expr, $v46:expr, $v47:expr,
        $v48:expr, $v49:expr, $v50:expr, $v51:expr, $v52:expr, $v53:expr, $v54:expr, $v55:expr,
        $v56:expr, $v57:expr, $v58:expr, $v59:expr, $v60:expr, $v61:expr, $v62:expr, $v63:expr
        $(,)*
    ) => {[
        $($sorted),*,
        $v0, $v32, $v8, $v40, $v4, $v36, $v12, $v44, $v2, $v34, $v10, $v42, $v6, $v38, $v14,
        $v46, $v1, $v33, $v9, $v41, $v5, $v37, $v13, $v45, $v3, $v35, $v11, $v43, $v7, $v39,
        $v15, $v47, $v16, $v48, $v24, $v56, $v20, $v52, $v28, $v60, $v18, $v50, $v26, $v58,
        $v22, $v54, $v30, $v62, $v17, $v49, $v25, $v57, $v21, $v53, $v29, $v61, $v19, $v51,
        $v27, $v59, $v23, $v55, $v31, $v63,
    ]};

}

mod bits {
    use std::ops::BitXor;
    #[cfg(test)]
    mod bits_tests {
        use super::*;

        #[test]
        fn test_to_string() {
            let b = gen_bits();
            assert_eq!(
                b.to_string(),
                String::from("0b0001001000110100010101100111100010000111011001010100001100100001")
            );
        }

        #[test]
        fn test_set_bit() {
            let mut b = gen_bits();

            b.set_bit(39, true);
            assert_eq!(
                b.to_string().as_str(),
                "0b0001001000110100010101101111100010000111011001010100001100100001"
            );
        }

        #[test]
        fn test_push_bit_he() {
            let mut b = Bits::new(0, 0);
            let some_bit = [0, 1, 0, 0, 1, 1, 0, 1, 0, 1];
            for i in some_bit.iter() {
                let value;
                if *i == 0 {
                    value = false;
                } else {
                    value = true
                }
                b.push_he(value);
            }
            assert_eq!(b.to_string().as_str(), "0b1010110010");
        }

        #[test]
        fn test_push_bits_he() {
            //0b0100001100100001
            let mut b = Bits {
                data: 0b10100111000110111101001000010011,
                width: 32,
            };
            let src = Bits {
                data: 0b00011111110100100011001111100011,
                width: 32,
            };
            b.pushn_he(&src);
            assert_eq!(
                b.to_string(),
                "0b0001111111010010001100111110001110100111000110111101001000010011"
            );
        }

        #[test]
        fn test_pop_bits_le() {
            let mut b = gen_bits();
            assert_eq!(b.popn_le(12).to_string(), "0b001100100001");
            assert_eq!(
                b.to_string(),
                "0b0001001000110100010101100111100010000111011001010100"
            );
        }

        #[test]
        fn test_get_bits() {
            let b = gen_bits();
            let some_pos: [u8; 26] = [
                60, 57, 53, 52, 50, 46, 44, 42, 41, 38, 37, 36, 35, 31, 26, 25, 24, 22, 21, 18, 16,
                14, 9, 8, 5, 0,
            ];
            let mut target = String::from("0b");
            target.push_str("1".repeat(some_pos.len()).as_str());
            assert_eq!(b.get_bits(&some_pos).to_string(), target);
        }

        #[test]
        fn test_get_bits_seq() {
            let b = gen_bits();
            let pos = 7;
            let width = 16;
            let crap = b.get_bits_seq(pos, width).to_string();

            let b_to_string = b.to_string();
            let mut target = String::from("0b");

            {
                let le = (b.width + 2 - pos - width) as usize;
                let re = (b.width + 2 - pos) as usize;
                target.push_str(b_to_string.as_str().get(le..re).unwrap());
            }
            //let target = String::from("0b1100101010000110");
            assert_eq!(crap, target);
        }

        #[test]
        fn test_shift_left_loop() {
            let mut b = Bits {
                data: 0b111111000000,
                width: 12,
            };
            b.shl_loop(15);
            assert_eq!(b.to_string(), "0b111000000111");
        }

        fn gen_bits() -> Bits {
            Bits {
                //"0b0001001000110100010101100111100010000111011001010100001100100001"
                data: 0x1234567887654321,
                width: 64,
            }
        }
    }

    #[derive(Clone)]
    pub struct Bits {
        data: u64,
        width: usize,
    }

    impl Bits {
        pub fn new(data: u64, width: usize) -> Bits {
            Bits { data, width }
        }

        pub fn from_u64(data: u64, width: usize) -> Bits {
            Bits { data, width }
        }

        pub fn to_u64(&self) -> u64 {
            self.data & (u64::MAX >> (64 - self.width))
        }

        //获得从低到高第pos位
        pub fn get_bit(&self, pos: usize) -> bool {
            assert_eq!(pos < self.width, true);
            if (self.data >> pos) & 1 == 0 {
                false
            } else {
                true
            }
        }

        //设置从低到高第pos位
        pub fn set_bit(&mut self, pos: usize, value: bool) {
            assert_eq!(pos < self.width, true);
            self.data &= !(1_u64 << pos);
            self.data |= (value as u64) << pos;
        }

        //把一个bit压到高位
        pub fn push_he(&mut self, value: bool) {
            let value = value as u64;
            if self.width == 64 {
                self.data >>= 1;
            }
            self.data |= value << self.width;
            if self.width != 64 {
                self.width += 1;
            }
        }

        //把src的所有bit放到self的高位
        pub fn pushn_he(&mut self, src: &Bits) {
            if self.width + src.width >= 64 {
                self.data >>= src.width + self.width - 64;
                self.data |= src.data << (64 - src.width);
                self.width = 64;
            } else {
                self.data |= src.data << self.width;
                self.width = src.width + self.width;
            }
        }

        //从self的低位pop出len位
        pub fn popn_le(&mut self, len: usize) -> Bits {
            assert_eq!(self.width >= len, true);
            let new_bits = Bits {
                data: self.data & !(u64::MAX << len),
                width: len,
            };
            self.width -= len;
            self.data >>= len;
            new_bits
        }

        //根据table提供的映射，返回一个新的Bits
        pub fn get_bits(&self, table: &[u8]) -> Bits {
            assert_ne!(self.width, 0);
            let mut new_bits = Bits { data: 0, width: 0 };
            for i in table {
                assert_eq!((*i as usize) < self.width, true);
                new_bits.push_he(self.get_bit(i.clone() as usize));
            }
            new_bits
        }

        //把从pos(低位)到pos+width(高位)的位放进新的Bits中
        pub fn get_bits_seq(&self, pos: usize, width: usize) -> Bits {
            assert_ne!(width, 0);
            assert_eq!(pos + width <= self.width, true);
            Bits {
                data: (self.data >> pos) & !(u64::MAX << width),
                width,
            }
        }

        //左移step位，被移出width的位放到低位
        pub fn shl_loop(&mut self, step: usize) {
            let step = step % self.width;
            //将width以外的位清零
            self.data &= !(u64::MAX << self.width);
            self.data = (self.data << step) | (self.data >> (self.width - step))

            // //返回一个Bits结构体
            // let data = self.data & !(u64::MAX << self.width);
            // Bits {
            //     data: (data << step) | (data >> (self.width - step)),
            //     width: self.width,
            // }
        }

        pub fn bitxor(&self, rhs: &Bits) -> Bits {
            assert_eq!(self.width, rhs.width);
            Bits {
                data: self.data ^ rhs.data,
                width: self.width,
            }
        }
    }

    impl ToString for Bits {
        //高位在前
        fn to_string(&self) -> String {
            let mut s = String::from("0b");
            for i in 0..self.width {
                s.push_str(
                    (self.get_bit(self.width - 1 - i) as u8)
                        .to_string()
                        .as_str(),
                );
            }
            s
        }
    }

    impl BitXor for Bits {
        type Output = Self;

        //this func consumes self and argv
        fn bitxor(self, rhs: Self) -> Self::Output {
            assert_eq!(self.width, rhs.width);
            Bits {
                data: self.data ^ rhs.data,
                width: self.width,
            }
        }
    }
}

mod des_constants {
    pub const IP: [u8; 64] = array_all_dec_1![
        58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14,
        6, 64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11,
        3, 61, 53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7
    ];

    pub const KEY_SHIFTS: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

    pub const PC1: [u8; 56] = array_all_dec_1![
        57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3,
        60, 52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45,
        37, 29, 21, 13, 5, 28, 20, 12, 4
    ];

    pub const PC2: [u8; 48] = array_all_dec_1![
        14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41,
        52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32
    ];

    pub const EBOX: [u8; 48] = array_all_dec_1![
        32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17, 16, 17,
        18, 19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1
    ];

    pub const SBOX: [u8; 512] = sort_the_fucking_sbox!([
        14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7, 0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12,
        11, 9, 5, 3, 8, 4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0, 15, 12, 8, 2, 4, 9,
        1, 7, 5, 11, 3, 14, 10, 0, 6, 13, 15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10, 3,
        13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5, 0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6,
        9, 3, 2, 15, 13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9, 10, 0, 9, 14, 6, 3, 15,
        5, 1, 13, 12, 7, 11, 4, 2, 8, 13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1, 13, 6,
        4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7, 1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11,
        5, 2, 12, 7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15, 13, 8, 11, 5, 6, 15, 0, 3,
        4, 7, 2, 12, 1, 10, 14, 9, 10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4, 3, 15, 0,
        6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14, 2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0,
        14, 9, 14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6, 4, 2, 1, 11, 10, 13, 7, 8, 15,
        9, 12, 5, 6, 3, 0, 14, 11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3, 12, 1, 10, 15,
        9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11, 10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3,
        8, 9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6, 4, 3, 2, 12, 9, 5, 15, 10, 11, 14,
        1, 7, 6, 0, 8, 13, 4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1, 13, 0, 11, 7, 4,
        9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6, 1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2,
        6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12, 13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3,
        14, 5, 0, 12, 7, 1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2, 7, 11, 4, 1, 9, 12,
        14, 2, 0, 6, 10, 13, 15, 3, 5, 8, 2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11,
    ]);

    pub const PBOX: [u8; 32] = array_all_dec_1![
        16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27, 3, 9,
        19, 13, 30, 6, 22, 11, 4, 25
    ];

    pub const FP: [u8; 64] = array_all_dec_1![
        40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62,
        30, 37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19,
        59, 27, 34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25
    ];
}
