pub mod sha2 {
    pub mod _256 {
        // pre determined constants.
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
        ];


        // initial hash values
        const H: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
        ];

        /// Generate SHA256 hash accprding to the original specification
        pub fn sha256(message: &[u8]) -> Vec<u8> {

            // number of bytes
            let length: usize = message.len() * 8;
            let mut message: Vec<u8> = message.to_vec();
        
            message.push(0x80);
        
            while (message.len() * 8 + 64) % 512 != 0 {
                message.push(0x00);
            }
        
            // pad 8 bytes or 64 bits
            message.extend_from_slice(&(length as u64).to_be_bytes());
        
            assert_eq!(message.len() * 8 % 512, 0, "Padding incorrect?: {}", message.len());
        

            // parsing, contains 64 bytes or 512 bit chunks of initial message
            let blocks: Vec<&[u8]> = message.chunks(64).collect();
        
            let mut h_: [u32; 8] = H;
            let mut w: [u32; 64] = [0; 64];
            let mut a: u32;
            let mut b: u32;
            let mut c: u32;
            let mut d: u32;
            let mut e: u32;
            let mut f: u32;
            let mut g: u32;
            let mut h: u32;
        
            // hash computation
            for message_block in blocks {
                for t in 0..16 {
                    // add t-nth 32bit word of the block, 8 bytes at a time
                    w[t] = u32::from_be_bytes(message_block[t * 4..(t + 1) * 4].try_into().unwrap());
                }
        
                for t in 16..64 {
                    let term1: u32 = sig1(w[t - 2]);
                    let term2: u32 = w[t - 7];
                    let term3: u32 = sig0(w[t - 15]);
                    let term4: u32 = w[t - 16];
        
                    // append 4 bytes
                    w[t] = modulo_2p32(term1.wrapping_add(term2).wrapping_add(term3).wrapping_add(term4));
                }
        
                // initalise working variables
                a = h_[0];
                b = h_[1];
                c = h_[2];
                d = h_[3];
                e = h_[4];
                f = h_[5];
                g = h_[6];
                h = h_[7];
        
                for t in 0..64 {
                    let t1: u32 = h.wrapping_add(ep1(e)).wrapping_add(ch(e, f, g)).wrapping_add(K[t]).wrapping_add(w[t]);
                    let t2: u32 = ep0(a).wrapping_add(maj(a, b, c));
        
                    h = g;
                    g = f;
                    f = e;
                    e = d.wrapping_add(t1);
                    d = c;
                    c = b;
                    b = a;
                    a = t1.wrapping_add(t2);
                }
        
                // intermediate hash value
                h_[0] = h_[0].wrapping_add(a);
                h_[1] = h_[1].wrapping_add(b);
                h_[2] = h_[2].wrapping_add(c);
                h_[3] = h_[3].wrapping_add(d);
                h_[4] = h_[4].wrapping_add(e);
                h_[5] = h_[5].wrapping_add(f);
                h_[6] = h_[6].wrapping_add(g);
                h_[7] = h_[7].wrapping_add(h);
            }
        
            h_.iter()
                .flat_map(|&x| x.to_be_bytes().to_vec())
                .collect()
        }

        pub fn digest(array: &[u8]) -> String {
            let mut digest_string: String = String::new();
        
            for value in array {
                for ch in format!("{:x}", value).chars() {
                    digest_string.push(ch)
                }
            }
        
            digest_string
        }

        fn rot_right(num: u32, shift: u32) -> u32 {
            num.wrapping_shr(shift) | num.wrapping_shl(32_u32.wrapping_sub(shift))
        }

        fn ch(x: u32, y: u32, z: u32) -> u32 {
            (x & y) ^ (!x & z)
        }

        fn maj(x: u32, y: u32, z: u32) -> u32 {
            (x & y) ^ (x & z) ^ (y & z)
        }

        fn ep0(x: u32) -> u32 {
            rot_right(x, 2) ^ rot_right(x, 13) ^ rot_right(x, 22)
        }

        fn ep1(x: u32) -> u32 {
            rot_right(x, 6) ^ rot_right(x, 11) ^ rot_right(x, 25)
        }

        fn sig0(x: u32) -> u32 {
            rot_right(x, 7) ^ rot_right(x, 18) ^ (x >> 3)
        }

        fn sig1(x: u32) -> u32 {
            rot_right(x, 17) ^ rot_right(x, 19) ^ (x >> 10)
        }

        fn modulo_2p32(num: u32) -> u32 {
            (num as u64 % 0x100000000) as u32
        }
    }
}