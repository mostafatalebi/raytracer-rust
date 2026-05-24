use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::vector::colors::NColor3;
use crate::vector::vec4f::Vec4f;

fn random_1_to_n(n: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=n)
}

pub fn pick_random_color() -> NColor3 {
    let mut colors:[Vec4f; 100] = [Vec4f::default(); 100];
    colors[0] = Vec4f::new(42.0, 188.0, 73.0, 1.0);
    colors[1] = Vec4f::new(215.0, 34.0, 161.0, 1.0);
    colors[2] = Vec4f::new(89.0, 241.0, 203.0, 1.0);
    colors[3] = Vec4f::new(14.0, 92.0, 255.0, 1.0);
    colors[4] = Vec4f::new(183.0, 112.0, 49.0, 1.0);
    colors[5] = Vec4f::new(201.0, 250.0, 12.0, 1.0);
    colors[6] = Vec4f::new(76.0, 153.0, 192.0, 1.0);
    colors[7] = Vec4f::new(132.0, 8.0, 114.0, 1.0);
    colors[8] = Vec4f::new(244.0, 169.0, 31.0, 1.0);
    colors[9] = Vec4f::new(60.0, 222.0, 140.0, 1.0);

    colors[10] = Vec4f::new(105.0, 41.0, 230.0, 1.0);
    colors[11] = Vec4f::new(228.0, 95.0, 85.0, 1.0);
    colors[12] = Vec4f::new(33.0, 174.0, 219.0, 1.0);
    colors[13] = Vec4f::new(158.0, 206.0, 52.0, 1.0);
    colors[14] = Vec4f::new(91.0, 13.0, 168.0, 1.0);
    colors[15] = Vec4f::new(252.0, 118.0, 180.0, 1.0);
    colors[16] = Vec4f::new(5.0, 237.0, 99.0, 1.0);
    colors[17] = Vec4f::new(119.0, 82.0, 247.0, 1.0);
    colors[18] = Vec4f::new(199.0, 147.0, 63.0, 1.0);
    colors[19] = Vec4f::new(27.0, 110.0, 151.0, 1.0);

    colors[20] = Vec4f::new(142.0, 253.0, 18.0, 1.0);
    colors[21] = Vec4f::new(88.0, 66.0, 121.0, 1.0);
    colors[22] = Vec4f::new(211.0, 19.0, 204.0, 1.0);
    colors[23] = Vec4f::new(165.0, 189.0, 242.0, 1.0);
    colors[24] = Vec4f::new(54.0, 225.0, 71.0, 1.0);
    colors[25] = Vec4f::new(123.0, 48.0, 9.0, 1.0);
    colors[26] = Vec4f::new(239.0, 134.0, 155.0, 1.0);
    colors[27] = Vec4f::new(70.0, 161.0, 220.0, 1.0);
    colors[28] = Vec4f::new(181.0, 90.0, 254.0, 1.0);
    colors[29] = Vec4f::new(10.0, 204.0, 133.0, 1.0);

    colors[30] = Vec4f::new(208.0, 57.0, 44.0, 1.0);
    colors[31] = Vec4f::new(96.0, 141.0, 185.0, 1.0);
    colors[32] = Vec4f::new(149.0, 233.0, 26.0, 1.0);
    colors[33] = Vec4f::new(47.0, 78.0, 213.0, 1.0);
    colors[34] = Vec4f::new(226.0, 115.0, 172.0, 1.0);
    colors[35] = Vec4f::new(111.0, 194.0, 80.0, 1.0);
    colors[36] = Vec4f::new(176.0, 23.0, 137.0, 1.0);
    colors[37] = Vec4f::new(63.0, 249.0, 251.0, 1.0);
    colors[38] = Vec4f::new(137.0, 102.0, 58.0, 1.0);
    colors[39] = Vec4f::new(245.0, 170.0, 11.0, 1.0);

    colors[40] = Vec4f::new(19.0, 128.0, 92.0, 1.0);
    colors[41] = Vec4f::new(154.0, 50.0, 229.0, 1.0);
    colors[42] = Vec4f::new(218.0, 212.0, 74.0, 1.0);
    colors[43] = Vec4f::new(82.0, 166.0, 143.0, 1.0);
    colors[44] = Vec4f::new(130.0, 84.0, 198.0, 1.0);
    colors[45] = Vec4f::new(240.0, 39.0, 120.0, 1.0);
    colors[46] = Vec4f::new(51.0, 217.0, 37.0, 1.0);
    colors[47] = Vec4f::new(171.0, 145.0, 252.0, 1.0);
    colors[48] = Vec4f::new(94.0, 109.0, 61.0, 1.0);
    colors[49] = Vec4f::new(22.0, 182.0, 231.0, 1.0);

    colors[50] = Vec4f::new(195.0, 73.0, 14.0, 1.0);
    colors[51] = Vec4f::new(113.0, 244.0, 163.0, 1.0);
    colors[52] = Vec4f::new(67.0, 32.0, 116.0, 1.0);
    colors[53] = Vec4f::new(253.0, 126.0, 207.0, 1.0);
    colors[54] = Vec4f::new(144.0, 191.0, 48.0, 1.0);
    colors[55] = Vec4f::new(36.0, 87.0, 246.0, 1.0);
    colors[56] = Vec4f::new(184.0, 224.0, 89.0, 1.0);
    colors[57] = Vec4f::new(101.0, 15.0, 79.0, 1.0);
    colors[58] = Vec4f::new(221.0, 162.0, 29.0, 1.0);
    colors[59] = Vec4f::new(50.0, 202.0, 118.0, 1.0);

    colors[60] = Vec4f::new(159.0, 61.0, 238.0, 1.0);
    colors[61] = Vec4f::new(236.0, 99.0, 68.0, 1.0);
    colors[62] = Vec4f::new(28.0, 178.0, 214.0, 1.0);
    colors[63] = Vec4f::new(129.0, 248.0, 41.0, 1.0);
    colors[64] = Vec4f::new(78.0, 43.0, 152.0, 1.0);
    colors[65] = Vec4f::new(247.0, 139.0, 181.0, 1.0);
    colors[66] = Vec4f::new(9.0, 229.0, 102.0, 1.0);
    colors[67] = Vec4f::new(117.0, 72.0, 250.0, 1.0);
    colors[68] = Vec4f::new(202.0, 151.0, 55.0, 1.0);
    colors[69] = Vec4f::new(62.0, 119.0, 160.0, 1.0);

    colors[70] = Vec4f::new(145.0, 254.0, 23.0, 1.0);
    colors[71] = Vec4f::new(86.0, 59.0, 127.0, 1.0);
    colors[72] = Vec4f::new(214.0, 25.0, 197.0, 1.0);
    colors[73] = Vec4f::new(162.0, 185.0, 243.0, 1.0);
    colors[74] = Vec4f::new(57.0, 221.0, 77.0, 1.0);
    colors[75] = Vec4f::new(120.0, 52.0, 13.0, 1.0);
    colors[76] = Vec4f::new(242.0, 131.0, 149.0, 1.0);
    colors[77] = Vec4f::new(69.0, 167.0, 223.0, 1.0);
    colors[78] = Vec4f::new(179.0, 93.0, 251.0, 1.0);
    colors[79] = Vec4f::new(15.0, 201.0, 136.0, 1.0);

    colors[80] = Vec4f::new(205.0, 54.0, 40.0, 1.0);
    colors[81] = Vec4f::new(93.0, 144.0, 182.0, 1.0);
    colors[82] = Vec4f::new(152.0, 230.0, 31.0, 1.0);
    colors[83] = Vec4f::new(44.0, 81.0, 210.0, 1.0);
    colors[84] = Vec4f::new(229.0, 112.0, 175.0, 1.0);
    colors[85] = Vec4f::new(108.0, 197.0, 83.0, 1.0);
    colors[86] = Vec4f::new(173.0, 20.0, 140.0, 1.0);
    colors[87] = Vec4f::new(66.0, 246.0, 248.0, 1.0);
    colors[88] = Vec4f::new(134.0, 105.0, 53.0, 1.0);
    colors[89] = Vec4f::new(248.0, 167.0, 16.0, 1.0);

    colors[90] = Vec4f::new(16.0, 131.0, 95.0, 1.0);
    colors[91] = Vec4f::new(157.0, 47.0, 226.0, 1.0);
    colors[92] = Vec4f::new(211.0, 215.0, 71.0, 1.0);
    colors[93] = Vec4f::new(85.0, 163.0, 146.0, 1.0);
    colors[94] = Vec4f::new(127.0, 87.0, 195.0, 1.0);
    colors[95] = Vec4f::new(243.0, 42.0, 117.0, 1.0);
    colors[96] = Vec4f::new(48.0, 220.0, 34.0, 1.0);
    colors[97] = Vec4f::new(174.0, 142.0, 249.0, 1.0);
    colors[98] = Vec4f::new(91.0, 112.0, 64.0, 1.0);
    colors[99] = Vec4f::new(25.0, 179.0, 228.0, 1.0);

    colors.shuffle(&mut thread_rng());
    colors[random_1_to_n(99) as usize].to_3()
}

