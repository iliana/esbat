// Copyright (c) 2018 Edward M. Reingold and Nachum Dershowitz
// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.
//
// Tables from:
//
//    E. M. Reingold and N. Dershowitz, Calendrical Calculations: The Ultimate Edition.
//        Cambridge University Press, 2018. doi:10.1017/9781107415058

#![allow(clippy::unreadable_literal)]

// Table 14.1, "Values of the arguments `$\tilde{x}$`, `$\tilde{y}$`, and `$\tilde{z}$` in
// `solar-longitude`"
pub(crate) const SOLAR_LONGITUDE_TABLE: [(f64, f64, f64); 49] = [
    (403406.0, 270.54861, 0.9287892),
    (195207.0, 340.19128, 35999.1376958),
    (119433.0, 63.91854, 35999.4089666),
    (112392.0, 331.2622, 35998.7287385),
    (3891.0, 317.843, 71998.20261),
    (2819.0, 86.631, 71998.4403),
    (1721.0, 240.052, 36000.35726),
    (660.0, 310.26, 71997.4812),
    (350.0, 247.23, 32964.4678),
    (334.0, 260.87, -19.441),
    (314.0, 297.82, 445267.1117),
    (268.0, 343.14, 45036.884),
    (242.0, 166.79, 3.1008),
    (234.0, 81.53, 22518.4434),
    (158.0, 3.5, -19.9739),
    (132.0, 132.75, 65928.9345),
    (129.0, 182.95, 9038.0293),
    (114.0, 162.03, 3034.7684),
    (99.0, 29.8, 33718.148),
    (93.0, 266.4, 3034.448),
    (86.0, 249.2, -2280.773),
    (78.0, 157.6, 29929.992),
    (72.0, 257.8, 31556.493),
    (68.0, 185.1, 149.588),
    (64.0, 69.9, 9037.75),
    (46.0, 8.0, 107997.405),
    (38.0, 197.1, -4444.176),
    (37.0, 250.4, 151.771),
    (32.0, 65.3, 67555.316),
    (29.0, 162.7, 31556.08),
    (28.0, 341.5, -4561.54),
    (27.0, 291.6, 107996.706),
    (27.0, 98.5, 1221.655),
    (25.0, 146.7, 62894.167),
    (24.0, 110.0, 31437.369),
    (21.0, 5.2, 14578.298),
    (21.0, 342.6, -31931.757),
    (20.0, 230.9, 34777.243),
    (18.0, 256.1, 1221.999),
    (17.0, 45.3, 62894.511),
    (14.0, 242.9, -4442.039),
    (13.0, 115.2, 107997.909),
    (13.0, 151.8, 119.066),
    (13.0, 285.3, 16859.071),
    (12.0, 53.3, -4.578),
    (10.0, 126.6, 26895.292),
    (10.0, 205.7, -39.127),
    (10.0, 85.9, 12297.536),
    (10.0, 146.1, 90073.778),
];

// Table 14.3, "Values of the arguments `$\tilde{v}$`, `$\tilde{w}$`, `$\tilde{x}$`, `$\tilde{y}$`,
// and `$\tilde{z}$` in `nth-new-moon`"
pub(crate) const NTH_NEW_MOON_CORRECTION_TABLE: [(f64, i32, f64, f64, f64); 24] = [
    (-0.4072, 0, 0.0, 1.0, 0.0),
    (0.17241, 1, 1.0, 0.0, 0.0),
    (0.01608, 0, 0.0, 2.0, 0.0),
    (0.01039, 0, 0.0, 0.0, 2.0),
    (0.00739, 1, -1.0, 1.0, 0.0),
    (-0.00514, 1, 1.0, 1.0, 0.0),
    (0.00208, 2, 2.0, 0.0, 0.0),
    (-0.00111, 0, 0.0, 1.0, -2.0),
    (-0.00057, 0, 0.0, 1.0, 2.0),
    (0.00056, 1, 1.0, 2.0, 0.0),
    (-0.00042, 0, 0.0, 3.0, 0.0),
    (0.00042, 1, 1.0, 0.0, 2.0),
    (0.00038, 1, 1.0, 0.0, -2.0),
    (-0.00024, 1, -1.0, 2.0, 0.0),
    (-0.00007, 0, 2.0, 1.0, 0.0),
    (0.00004, 0, 0.0, 2.0, -2.0),
    (0.00004, 0, 3.0, 0.0, 0.0),
    (0.00003, 0, 1.0, 1.0, -2.0),
    (0.00003, 0, 0.0, 2.0, 2.0),
    (-0.00003, 0, 1.0, 1.0, 2.0),
    (0.00003, 0, -1.0, 1.0, 2.0),
    (-0.00002, 0, -1.0, 1.0, -2.0),
    (-0.00002, 0, 1.0, 3.0, 0.0),
    (0.00002, 0, 0.0, 4.0, 0.0),
];

// Table 14.4, "Values of the arguments `$\tilde{i}$`, `$\tilde{j}$`, and `$\tilde{l}$`, in
// `nth-new-moon`"
pub(crate) const NTH_NEW_MOON_ADDITIONAL_TABLE: [(f64, f64, f64); 13] = [
    (251.88, 0.016321, 0.000165),
    (251.83, 26.651886, 0.000164),
    (349.42, 36.412478, 0.000126),
    (84.66, 18.206239, 0.00011),
    (141.74, 53.303771, 0.000062),
    (207.14, 2.453732, 0.00006),
    (154.84, 7.30686, 0.000056),
    (34.52, 27.261239, 0.000047),
    (207.19, 0.121824, 0.000042),
    (291.34, 1.844379, 0.00004),
    (161.72, 24.198154, 0.000037),
    (239.56, 25.513099, 0.000035),
    (331.55, 3.592518, 0.000023),
];

// Table 14.5, "Values of the arguments `$\tilde{v}$`, `$\tilde{w}$`, `$\tilde{x}$`, `$\vec{y}$`,
// and `$\vec{z}$`, in `lunar-longitude`"
pub(crate) const LUNAR_LONGITUDE_CORRECTION_TABLE: [(f64, f64, i32, f64, f64); 59] = [
    (6288774.0, 0.0, 0, 1.0, 0.0),
    (1274027.0, 2.0, 0, -1.0, 0.0),
    (658314.0, 2.0, 0, 0.0, 0.0),
    (213618.0, 0.0, 0, 2.0, 0.0),
    (-185116.0, 0.0, 1, 0.0, 0.0),
    (-114332.0, 0.0, 0, 0.0, 2.0),
    (58793.0, 2.0, 0, -2.0, 0.0),
    (57066.0, 2.0, -1, -1.0, 0.0),
    (53322.0, 2.0, 0, 1.0, 0.0),
    (45758.0, 2.0, -1, 0.0, 0.0),
    (-40923.0, 0.0, 1, -1.0, 0.0),
    (-34720.0, 1.0, 0, 0.0, 0.0),
    (-30383.0, 0.0, 1, 1.0, 0.0),
    (15327.0, 2.0, 0, 0.0, -2.0),
    (-12528.0, 0.0, 0, 1.0, 2.0),
    (10980.0, 0.0, 0, 1.0, -2.0),
    (10675.0, 4.0, 0, -1.0, 0.0),
    (10034.0, 0.0, 0, 3.0, 0.0),
    (8548.0, 4.0, 0, -2.0, 0.0),
    (-7888.0, 2.0, 1, -1.0, 0.0),
    (-6766.0, 2.0, 1, 0.0, 0.0),
    (-5163.0, 1.0, 0, -1.0, 0.0),
    (4987.0, 1.0, 1, 0.0, 0.0),
    (4036.0, 2.0, -1, 1.0, 0.0),
    (3994.0, 2.0, 0, 2.0, 0.0),
    (3861.0, 4.0, 0, 0.0, 0.0),
    (3665.0, 2.0, 0, -3.0, 0.0),
    (-2689.0, 0.0, 1, -2.0, 0.0),
    (-2602.0, 2.0, 0, -1.0, 2.0),
    (2390.0, 2.0, -1, -2.0, 0.0),
    (-2348.0, 1.0, 0, 1.0, 0.0),
    (2236.0, 2.0, -2, 0.0, 0.0),
    (-2120.0, 0.0, 1, 2.0, 0.0),
    (-2069.0, 0.0, 2, 0.0, 0.0),
    (2048.0, 2.0, -2, -1.0, 0.0),
    (-1773.0, 2.0, 0, 1.0, -2.0),
    (-1595.0, 2.0, 0, 0.0, 2.0),
    (1215.0, 4.0, -1, -1.0, 0.0),
    (-1110.0, 0.0, 0, 2.0, 2.0),
    (-892.0, 3.0, 0, -1.0, 0.0),
    (-810.0, 2.0, 1, 1.0, 0.0),
    (759.0, 4.0, -1, -2.0, 0.0),
    (-713.0, 0.0, 2, -1.0, 0.0),
    (-700.0, 2.0, 2, -1.0, 0.0),
    (691.0, 2.0, 1, -2.0, 0.0),
    (596.0, 2.0, -1, 0.0, -2.0),
    (549.0, 4.0, 0, 1.0, 0.0),
    (537.0, 0.0, 0, 4.0, 0.0),
    (520.0, 4.0, -1, 0.0, 0.0),
    (-487.0, 1.0, 0, -2.0, 0.0),
    (-399.0, 2.0, 1, 0.0, -2.0),
    (-381.0, 0.0, 0, 2.0, -2.0),
    (351.0, 1.0, 1, 1.0, 0.0),
    (-340.0, 3.0, 0, -2.0, 0.0),
    (330.0, 4.0, 0, -3.0, 0.0),
    (327.0, 2.0, -1, 2.0, 0.0),
    (-323.0, 0.0, 2, 1.0, 0.0),
    (299.0, 1.0, 1, -1.0, 0.0),
    (294.0, 2.0, 0, 3.0, 0.0),
];

// Subset of Appendix C: Sample Data
//
// |      |     Gregorian      | Ephemeris  | Solar longitude |   Lunar   | New moon at |
// | R.D. | year | month | day | correction |     at noon     | longitude |  or after   |
#[cfg(test)]
#[allow(clippy::type_complexity)]
pub(crate) const TEST_DATA: [(f64, (i32, u32, u32), f64, f64, f64, f64); 33] = [
    (-214193.0, (-586, 7, 24), 0.214169, 119.473431, 244.853905, -214174.605828),
    (-61387.0, (-168, 12, 5), 0.143632, 254.248961, 208.856738, -61382.995328),
    (25469.0, (70, 9, 24), 0.114444, 181.435996, 213.746842, 25495.809776),
    (49217.0, (135, 10, 2), 0.107183, 188.663922, 292.046243, 49238.502448),
    (171307.0, (470, 1, 8), 0.069498, 289.091566, 156.819014, 171318.435313),
    (210155.0, (576, 5, 20), 0.057506, 59.119741, 108.055632, 210180.691849),
    (253427.0, (694, 11, 10), 0.044758, 228.314554, 39.356097, 253442.859367),
    (369740.0, (1013, 4, 25), 0.017397, 34.460769, 98.565851, 369763.746413),
    (400085.0, (1096, 5, 24), 0.012796, 63.187995, 332.958296, 400091.578343),
    (434355.0, (1190, 3, 23), 0.008869, 2.457591, 92.259651, 434376.578106),
    (452605.0, (1240, 3, 10), 0.007262, 350.475934, 78.132029, 452627.191972),
    (470160.0, (1288, 4, 2), 0.005979, 13.49822, 274.946995, 470167.57836),
    (473837.0, (1298, 4, 27), 0.00574, 37.40392, 128.362844, 473858.853276),
    (507850.0, (1391, 6, 12), 0.003875, 81.02813, 89.51845, 507878.666842),
    (524156.0, (1436, 2, 3), 0.003157, 313.860498, 24.607322, 524179.247062),
    (544676.0, (1492, 4, 9), 0.002393, 19.95443, 53.485956, 544702.753873),
    (567118.0, (1553, 9, 19), 0.001731, 176.059431, 187.89852, 567146.513181),
    (569477.0, (1560, 3, 5), 0.001669, 344.922951, 320.172362, 569479.203258),
    (601716.0, (1648, 6, 10), 0.000615, 79.964921, 314.042566, 601727.033557),
    (613424.0, (1680, 6, 30), 0.000177, 99.302317, 145.474065, 613449.762129),
    (626596.0, (1716, 7, 24), 0.000101, 121.535304, 185.030507, 626620.369801),
    (645554.0, (1768, 6, 19), 0.000171, 88.567428, 142.189132, 645579.076748),
    (664224.0, (1819, 8, 2), 0.000136, 129.289884, 253.743375, 664242.886718),
    (671401.0, (1839, 3, 27), 0.000061, 6.14691, 151.648685, 671418.970538),
    (694799.0, (1903, 4, 19), 0.000014, 28.251993, 287.987743, 694807.563371),
    (704424.0, (1929, 8, 25), 0.000276, 151.780633, 25.626707, 704433.491182),
    (708842.0, (1941, 9, 29), 0.000296, 185.945867, 290.2883, 708863.597),
    (709409.0, (1943, 4, 19), 0.000302, 28.555607, 189.913142, 709424.404929),
    (709580.0, (1943, 10, 7), 0.000302, 193.347892, 284.93173, 709602.082686),
    (727274.0, (1992, 3, 17), 0.000675, 357.151254, 152.339044, 727291.2094),
    (728714.0, (1996, 2, 25), 0.000712, 336.170692, 51.662265, 728737.447691),
    (744313.0, (2038, 11, 10), 0.000963, 228.184879, 26.68206, 744329.573999),
    (764652.0, (2094, 7, 18), 0.002913, 116.439352, 175.500822, 764676.191273),
];
