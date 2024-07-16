fn rol_64(num: u128, shift: u128) -> u128 {
    (num >> (64 - (shift & 0x3F))) + (num << (shift & 0x3F)) % (1 << 64)
}

fn load_64(data: &[u128]) -> u128 {
    data.iter()
        .enumerate()
        .fold(0, |acc: u128, (i, curr)| acc + (curr << (8 * i)))
}

fn store_64(num: u128) -> Vec<u128> {
    (0..8).map(|i| (num >> (i << 3)) % 256).collect()
}

fn keccak_f1600_on_lanes(lanes: &mut [Vec<u128>]) {
    let mut r: u128 = 1;

    for _ in 0..24 {
        let c: Vec<u128> = (0..5)
            .map(|x| lanes[x][0] ^ lanes[x][1] ^ lanes[x][2] ^ lanes[x][3] ^ lanes[x][4])
            .collect();

        let d: Vec<u128> = (0..5)
            .map(|x| c[(x + 4) % 5] ^ rol_64(c[(x + 1) % 5], 1))
            .collect();

        for x in 0..5 {
            for y in 0..5 {
                lanes[x][y] ^= d[x]
            }
        }

        let mut x: usize = 1;
        let mut y: usize = 0;

        let mut tx: usize;
        let mut tc: u128;

        let mut current: u128 = lanes[x][y];

        for i in 0..24 {
            tx = x;
            x = y;
            y = (2 * tx + 3 * y) % 5;

            tc = current;
            current = lanes[x][y];
            lanes[x][y] = rol_64(tc, (i + 1) * (i + 2) / 2)
        }

        for y in 0..5 {
            let tl: Vec<u128> = (0..5).map(|ix| lanes[ix][y]).collect();

            for x in 0..5 {
                lanes[x][y] = tl[x] ^ (!tl[(x + 1) % 5] & tl[(x + 2) % 5])
            }
        }

        for i in 0..7 {
            r = ((r << 1) ^ ((r >> 7) * 0x71)) % 256;

            if r & 2 == 2 {
                lanes[0][0] ^= 1 << ((1 << i) - 1)
            }
        }
    }
}

fn keccak_f1600(state: &mut [u128]) {
    let mut lanes: Vec<Vec<u128>> = Vec::with_capacity(5);

    let mut index: usize;

    for x in 0..5 {
        let mut row: Vec<u128> = Vec::with_capacity(5);

        for y in 0..5 {
            index = 8 * (x + 5 * y);

            row.push(load_64(&state[index..(index + 8)]))
        }

        lanes.push(row)
    }

    keccak_f1600_on_lanes(&mut lanes);

    for x in 0..5 {
        let mut values: Vec<u128>;

        for y in 0..5 {
            values = store_64(lanes[x][y]);

            for (i, value) in values.iter().enumerate() {
                state[8 * (x + 5 * y) + i] = *value
            }
        }
    }
}

pub fn keccak(
    rate_in_bits: u128,
    capacity: u128,
    input_bytes: &[u128],
    delimited_suffix: u128,
    output_byte_len: u128,
) -> Vec<u128> {
    let mut output_byte_len: u128 = output_byte_len;

    let mut output_bytes: Vec<u128> = Vec::with_capacity(
        output_byte_len
            .try_into()
            .expect("Too high output byte len"),
    );
    let mut state: Vec<u128> = vec![0; 200];

    let rate_in_bytes: u128 = rate_in_bits / 8;

    let mut block_size: usize = 0;
    let mut input_offset: usize = 0;

    if (rate_in_bits + capacity) != 1600 || rate_in_bits & 7 != 0 {
        // % 8 -> & 7
        panic!(
            "Invalid args: ribytes + cap = {} != 1600; ribits = {} // 8 != 0",
            rate_in_bytes + capacity,
            rate_in_bits
        )
    }

    while input_offset < input_bytes.len() {
        block_size = (input_bytes.len() - input_offset).min(rate_in_bytes as usize);

        for i in 0..block_size {
            state[i] ^= input_bytes[input_offset + i]
        }

        input_offset += block_size;

        if block_size == rate_in_bytes as usize {
            keccak_f1600(&mut state);
            block_size = 0;
        }
    }

    state[block_size] ^= delimited_suffix;

    if (delimited_suffix & 0x80) != 0 && block_size as u128 == (rate_in_bytes - 1) {
        keccak_f1600(&mut state)
    }

    state[(rate_in_bytes - 1) as usize] ^= 0x80;

    keccak_f1600(&mut state);

    while output_byte_len > 0 {
        block_size = output_byte_len.min(rate_in_bytes) as usize;

        output_bytes.extend_from_slice(&state[..block_size]);

        output_byte_len -= block_size as u128;

        if output_byte_len > 0 {
            keccak_f1600(&mut state);
        }
    }

    output_bytes
}

/// it's a limited definition use [keccak] for more wide number limit.
pub fn keccak_u8(
    rate_in_bytes: u16,
    input_bytes: &[u8],
    delimited_suffix: u16,
    output_byte_len: u16,
) -> Vec<u8> {
    let rate_in_bits = (rate_in_bytes * 8) as u128;
    keccak(
        rate_in_bits,
        1600 - rate_in_bits,
        &input_bytes
            .iter()
            .map(|byte| *byte as u128)
            .collect::<Vec<u128>>(),
        delimited_suffix as u128,
        output_byte_len as u128,
    )
    .iter()
    .map(|byte| *byte as u8)
    .collect()
}

#[cfg(test)]
mod keccak_test {
    #[test]
    fn rol_64() {
        assert_eq!(super::rol_64(999, 999), 549206058074112);
        assert_eq!(super::rol_64(123456789, 0), 123456789);
        assert_eq!(super::rol_64(123456789, 64), 123456789);
        assert_eq!(super::rol_64(123456789, 128), 123456789);

        assert_eq!(super::rol_64(0xFFFFFFFFFFFFFFFF, 1), 0xFFFFFFFFFFFFFFFF);
        assert_eq!(super::rol_64(0xFFFFFFFFFFFFFFFF, 63), 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn load_64() {
        assert_eq!(super::load_64(&[0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(super::load_64(&[1, 1, 1, 1, 1, 1, 1, 1]), 72340172838076673);
        assert_eq!(
            super::load_64(&[1, 2, 3, 4, 5, 6, 7, 8]),
            578437695752307201
        );

        assert_eq!(
            super::load_64(&[255, 255, 255, 255, 255, 255, 255, 255]),
            18446744073709551615
        )
    }

    #[test]
    fn store_64() {
        assert_eq!(super::store_64(0), vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            super::store_64(72340172838076673),
            vec![1, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            super::store_64(578437695752307201),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(
            super::store_64(18446744073709551615),
            vec![255, 255, 255, 255, 255, 255, 255, 255]
        );
    }

    #[test]
    fn keccak_f1600_on_lanes() {
        let mut data_1: Vec<Vec<u128>> = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        let mut data_2: Vec<Vec<u128>> = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 0],
            vec![0, 0, 0, 0, 0],
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 0],
        ];

        super::keccak_f1600_on_lanes(&mut data_1);
        super::keccak_f1600_on_lanes(&mut data_2);

        assert_eq!(
            data_1,
            vec![
                vec![
                    17376452488221285863,
                    18417369716475457492,
                    16959053435453822517,
                    424854978622500449,
                    10668034807192757780
                ],
                vec![
                    9571781953733019530,
                    10448040663659726788,
                    12224711289652453635,
                    7259519967065370866,
                    1747952066141424100
                ],
                vec![
                    15391093639620504046,
                    10113917136857017974,
                    9342009439668884831,
                    7004910057750291985,
                    1654286879329379778
                ],
                vec![
                    13624874521033984333,
                    12479658147685402012,
                    4879704952849025062,
                    13293599522548616907,
                    8500057116360352059
                ],
                vec![
                    10027350355371872343,
                    3500241080921619556,
                    140226327413610143,
                    10105770293752443592,
                    16929593379567477321
                ]
            ]
        );

        assert_eq!(
            data_2,
            vec![
                vec![
                    6269942714399931890,
                    8933783005774876927,
                    14361267160555465972,
                    6505270177192387474,
                    2404738705400101649
                ],
                vec![
                    6807392100548068022,
                    15683499745133076168,
                    2764364400791029454,
                    2973048327548910661,
                    10255389283575608402
                ],
                vec![
                    7558976490878912777,
                    2924156069299676232,
                    5195951610477572339,
                    17254513324849854336,
                    17736849854948220342
                ],
                vec![
                    12256180026113095957,
                    7802670506913381763,
                    4066428769703078281,
                    18037914650129283290,
                    11984217004935637960
                ],
                vec![
                    1871358968484233706,
                    5803652345229422514,
                    17020757572326394724,
                    7947227104366760550,
                    17492745099396503234
                ]
            ]
        );
    }

    #[test]
    fn keccak_f1600() {
        fn build_vector_200() -> Vec<u128> {
            let mut v: Vec<u128> = Vec::with_capacity(200);

            for i in 0..200 {
                v.push(i)
            }

            v
        }

        let mut data = build_vector_200();

        super::keccak_f1600(&mut data);

        assert_eq!(
            data,
            vec![
                250, 124, 213, 218, 245, 145, 40, 18, 33, 41, 118, 220, 167, 229, 248, 184, 94,
                183, 117, 2, 140, 15, 172, 143, 53, 69, 49, 116, 150, 3, 238, 71, 44, 150, 140,
                203, 109, 168, 212, 23, 176, 60, 68, 181, 42, 167, 127, 14, 62, 40, 49, 107, 209,
                182, 175, 236, 9, 81, 188, 8, 52, 146, 3, 204, 59, 2, 229, 29, 148, 218, 98, 248,
                8, 156, 196, 242, 110, 157, 182, 149, 6, 23, 206, 158, 183, 172, 35, 85, 26, 222,
                120, 252, 36, 110, 0, 36, 178, 218, 25, 176, 6, 62, 11, 41, 180, 209, 47, 235, 46,
                65, 184, 227, 84, 182, 199, 44, 65, 170, 173, 49, 228, 183, 68, 75, 169, 186, 229,
                33, 157, 3, 92, 149, 142, 129, 220, 121, 67, 93, 49, 81, 189, 196, 28, 228, 194,
                64, 253, 228, 252, 160, 62, 124, 234, 97, 120, 54, 13, 53, 223, 13, 42, 243, 44,
                243, 163, 11, 202, 146, 221, 204, 119, 197, 2, 103, 137, 163, 222, 169, 188, 218,
                229, 194, 199, 111, 89, 65, 15, 246, 86, 132, 161, 15, 22, 174, 15, 227, 212, 129,
                8, 7
            ]
        )
    }

    #[test]
    fn keccak() {
        assert_eq!(
            super::keccak(1600, 0, &[0, 0, 0, 0], 0x00, 8),
            vec![97, 96, 30, 159, 251, 73, 67, 91]
        );
        assert_eq!(
            super::keccak(200, 1400, &[255, 255, 255, 255], 0xFF, 8),
            vec![242, 80, 39, 210, 73, 4, 71, 197]
        );

        assert_eq!(
            super::keccak(800, 800, &[], 0xFF, 16),
            vec![42, 230, 159, 117, 24, 0, 170, 245, 107, 29, 6, 88, 108, 163, 181, 71]
        );
        assert_eq!(
            super::keccak(1600, 0, &[], 0xFF, 16),
            vec![165, 7, 17, 179, 63, 250, 21, 160, 144, 85, 162, 243, 141, 75, 74, 46]
        );

        assert_eq!(
            super::keccak(1472, 128, &[], 0x00, 128),
            vec![
                1, 230, 255, 236, 50, 175, 7, 40, 116, 100, 103, 217, 240, 151, 122, 191, 145, 126,
                44, 169, 211, 42, 83, 76, 25, 204, 14, 174, 20, 113, 82, 182, 1, 117, 55, 187, 216,
                12, 120, 190, 216, 12, 252, 31, 104, 222, 239, 198, 208, 254, 111, 116, 102, 29,
                108, 172, 26, 159, 141, 33, 79, 37, 35, 104, 217, 116, 64, 71, 250, 139, 85, 3,
                230, 110, 229, 238, 193, 223, 134, 41, 86, 72, 138, 115, 90, 153, 69, 211, 196,
                116, 67, 211, 200, 208, 62, 238, 180, 18, 211, 230, 5, 103, 236, 110, 54, 251, 232,
                21, 248, 62, 204, 110, 185, 118, 203, 152, 92, 176, 49, 147, 26, 225, 158, 51, 180,
                190, 148, 176
            ]
        )
    }
}
