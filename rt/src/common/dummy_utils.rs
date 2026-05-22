use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::vector::colors::Rgba;

fn random_1_to_n(n: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=n)
}

pub fn pick_random_color() -> Rgba {
    let mut colors:[Rgba; 100] = [Rgba::default(); 100];
    colors[0] = Rgba::new(42.0, 188.0, 73.0, 1.0);
    colors[1] = Rgba::new(215.0, 34.0, 161.0, 1.0);
    colors[2] = Rgba::new(89.0, 241.0, 203.0, 1.0);
    colors[3] = Rgba::new(14.0, 92.0, 255.0, 1.0);
    colors[4] = Rgba::new(183.0, 112.0, 49.0, 1.0);
    colors[5] = Rgba::new(201.0, 250.0, 12.0, 1.0);
    colors[6] = Rgba::new(76.0, 153.0, 192.0, 1.0);
    colors[7] = Rgba::new(132.0, 8.0, 114.0, 1.0);
    colors[8] = Rgba::new(244.0, 169.0, 31.0, 1.0);
    colors[9] = Rgba::new(60.0, 222.0, 140.0, 1.0);

    colors[10] = Rgba::new(105.0, 41.0, 230.0, 1.0);
    colors[11] = Rgba::new(228.0, 95.0, 85.0, 1.0);
    colors[12] = Rgba::new(33.0, 174.0, 219.0, 1.0);
    colors[13] = Rgba::new(158.0, 206.0, 52.0, 1.0);
    colors[14] = Rgba::new(91.0, 13.0, 168.0, 1.0);
    colors[15] = Rgba::new(252.0, 118.0, 180.0, 1.0);
    colors[16] = Rgba::new(5.0, 237.0, 99.0, 1.0);
    colors[17] = Rgba::new(119.0, 82.0, 247.0, 1.0);
    colors[18] = Rgba::new(199.0, 147.0, 63.0, 1.0);
    colors[19] = Rgba::new(27.0, 110.0, 151.0, 1.0);

    colors[20] = Rgba::new(142.0, 253.0, 18.0, 1.0);
    colors[21] = Rgba::new(88.0, 66.0, 121.0, 1.0);
    colors[22] = Rgba::new(211.0, 19.0, 204.0, 1.0);
    colors[23] = Rgba::new(165.0, 189.0, 242.0, 1.0);
    colors[24] = Rgba::new(54.0, 225.0, 71.0, 1.0);
    colors[25] = Rgba::new(123.0, 48.0, 9.0, 1.0);
    colors[26] = Rgba::new(239.0, 134.0, 155.0, 1.0);
    colors[27] = Rgba::new(70.0, 161.0, 220.0, 1.0);
    colors[28] = Rgba::new(181.0, 90.0, 254.0, 1.0);
    colors[29] = Rgba::new(10.0, 204.0, 133.0, 1.0);

    colors[30] = Rgba::new(208.0, 57.0, 44.0, 1.0);
    colors[31] = Rgba::new(96.0, 141.0, 185.0, 1.0);
    colors[32] = Rgba::new(149.0, 233.0, 26.0, 1.0);
    colors[33] = Rgba::new(47.0, 78.0, 213.0, 1.0);
    colors[34] = Rgba::new(226.0, 115.0, 172.0, 1.0);
    colors[35] = Rgba::new(111.0, 194.0, 80.0, 1.0);
    colors[36] = Rgba::new(176.0, 23.0, 137.0, 1.0);
    colors[37] = Rgba::new(63.0, 249.0, 251.0, 1.0);
    colors[38] = Rgba::new(137.0, 102.0, 58.0, 1.0);
    colors[39] = Rgba::new(245.0, 170.0, 11.0, 1.0);

    colors[40] = Rgba::new(19.0, 128.0, 92.0, 1.0);
    colors[41] = Rgba::new(154.0, 50.0, 229.0, 1.0);
    colors[42] = Rgba::new(218.0, 212.0, 74.0, 1.0);
    colors[43] = Rgba::new(82.0, 166.0, 143.0, 1.0);
    colors[44] = Rgba::new(130.0, 84.0, 198.0, 1.0);
    colors[45] = Rgba::new(240.0, 39.0, 120.0, 1.0);
    colors[46] = Rgba::new(51.0, 217.0, 37.0, 1.0);
    colors[47] = Rgba::new(171.0, 145.0, 252.0, 1.0);
    colors[48] = Rgba::new(94.0, 109.0, 61.0, 1.0);
    colors[49] = Rgba::new(22.0, 182.0, 231.0, 1.0);

    colors[50] = Rgba::new(195.0, 73.0, 14.0, 1.0);
    colors[51] = Rgba::new(113.0, 244.0, 163.0, 1.0);
    colors[52] = Rgba::new(67.0, 32.0, 116.0, 1.0);
    colors[53] = Rgba::new(253.0, 126.0, 207.0, 1.0);
    colors[54] = Rgba::new(144.0, 191.0, 48.0, 1.0);
    colors[55] = Rgba::new(36.0, 87.0, 246.0, 1.0);
    colors[56] = Rgba::new(184.0, 224.0, 89.0, 1.0);
    colors[57] = Rgba::new(101.0, 15.0, 79.0, 1.0);
    colors[58] = Rgba::new(221.0, 162.0, 29.0, 1.0);
    colors[59] = Rgba::new(50.0, 202.0, 118.0, 1.0);

    colors[60] = Rgba::new(159.0, 61.0, 238.0, 1.0);
    colors[61] = Rgba::new(236.0, 99.0, 68.0, 1.0);
    colors[62] = Rgba::new(28.0, 178.0, 214.0, 1.0);
    colors[63] = Rgba::new(129.0, 248.0, 41.0, 1.0);
    colors[64] = Rgba::new(78.0, 43.0, 152.0, 1.0);
    colors[65] = Rgba::new(247.0, 139.0, 181.0, 1.0);
    colors[66] = Rgba::new(9.0, 229.0, 102.0, 1.0);
    colors[67] = Rgba::new(117.0, 72.0, 250.0, 1.0);
    colors[68] = Rgba::new(202.0, 151.0, 55.0, 1.0);
    colors[69] = Rgba::new(62.0, 119.0, 160.0, 1.0);

    colors[70] = Rgba::new(145.0, 254.0, 23.0, 1.0);
    colors[71] = Rgba::new(86.0, 59.0, 127.0, 1.0);
    colors[72] = Rgba::new(214.0, 25.0, 197.0, 1.0);
    colors[73] = Rgba::new(162.0, 185.0, 243.0, 1.0);
    colors[74] = Rgba::new(57.0, 221.0, 77.0, 1.0);
    colors[75] = Rgba::new(120.0, 52.0, 13.0, 1.0);
    colors[76] = Rgba::new(242.0, 131.0, 149.0, 1.0);
    colors[77] = Rgba::new(69.0, 167.0, 223.0, 1.0);
    colors[78] = Rgba::new(179.0, 93.0, 251.0, 1.0);
    colors[79] = Rgba::new(15.0, 201.0, 136.0, 1.0);

    colors[80] = Rgba::new(205.0, 54.0, 40.0, 1.0);
    colors[81] = Rgba::new(93.0, 144.0, 182.0, 1.0);
    colors[82] = Rgba::new(152.0, 230.0, 31.0, 1.0);
    colors[83] = Rgba::new(44.0, 81.0, 210.0, 1.0);
    colors[84] = Rgba::new(229.0, 112.0, 175.0, 1.0);
    colors[85] = Rgba::new(108.0, 197.0, 83.0, 1.0);
    colors[86] = Rgba::new(173.0, 20.0, 140.0, 1.0);
    colors[87] = Rgba::new(66.0, 246.0, 248.0, 1.0);
    colors[88] = Rgba::new(134.0, 105.0, 53.0, 1.0);
    colors[89] = Rgba::new(248.0, 167.0, 16.0, 1.0);

    colors[90] = Rgba::new(16.0, 131.0, 95.0, 1.0);
    colors[91] = Rgba::new(157.0, 47.0, 226.0, 1.0);
    colors[92] = Rgba::new(211.0, 215.0, 71.0, 1.0);
    colors[93] = Rgba::new(85.0, 163.0, 146.0, 1.0);
    colors[94] = Rgba::new(127.0, 87.0, 195.0, 1.0);
    colors[95] = Rgba::new(243.0, 42.0, 117.0, 1.0);
    colors[96] = Rgba::new(48.0, 220.0, 34.0, 1.0);
    colors[97] = Rgba::new(174.0, 142.0, 249.0, 1.0);
    colors[98] = Rgba::new(91.0, 112.0, 64.0, 1.0);
    colors[99] = Rgba::new(25.0, 179.0, 228.0, 1.0);

    colors.shuffle(&mut thread_rng());
    colors[random_1_to_n(99) as usize]
}

