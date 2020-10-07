// This program is copyright (C) 1982 by D. E. Knuth; all rights are reserved.

// TeX is a trademark of the American Mathematical Society.
// METAFONT is a trademark of Addison-Wesley Publishing Company.

// Although considerable effort has been expended to make the TeX program
// correct and reliable, no warranty is implied; the author disclaims any
// obligation or liability for damages, including but not limited to
// special, indirect, or consequential damages arising out of or in
// connection with the use or performance of this software. This work has
// been a ``labor of love'' and the author hopes that users enjoy it.

#![deny(warnings, missing_docs, missing_debug_implementations)]
#![allow(dead_code, non_upper_case_globals, non_camel_case_types, unused_imports)]
//! This is `TeX`, a document compiler intended to produce typesetting of high quality.

#[macro_use]
mod info;
mod pascal;

mod section_0001;
mod section_0002;
mod section_0003;
mod section_0004;
mod section_0005;
mod section_0006_to_0010;
mod section_0011_to_0016;
mod section_0017;
mod section_0018;
mod section_0019;
mod section_0020_to_0024;
mod section_0025;
mod section_0026_to_0031;
mod section_0032;
mod section_0033;
mod section_0034_to_0037;
mod section_0038_to_0053;
mod section_0054;
mod section_0055;
mod section_0056;
mod section_0057_to_0060;
#[macro_use]
mod section_0061;
mod section_0062_to_0071;
mod section_0072_to_0098;
mod section_0099_to_0100;
mod section_0101;
mod section_0102_to_0108;
mod section_0109;
mod section_0110;
mod section_0111;
mod section_0112;
mod section_0113;
mod section_0114;
mod section_0115_to_0132;
mod section_0133_to_0161;
mod section_0162_to_0172;
mod section_0173_to_0198;
mod section_0199_to_0202;
mod section_0203_to_0206;
mod section_0207_to_0210;
mod section_0211_to_0219;
mod section_0220_to_0255;
mod section_0256_to_0267;
mod section_0268_to_0288;
mod section_0289_to_0296;
mod section_0297_to_0299;
mod section_0300_to_0320;
mod section_0321_to_0331;
mod section_0332_to_0365;
mod section_0366_to_0401;
mod section_0402_to_0463;
mod section_0464_to_0486;
mod section_0487_to_0510;
mod section_0511_to_0538;
mod section_0539_to_0582;
mod section_0583_to_0591;
mod section_0592_to_0599;
mod section_0600_to_0643;
mod section_0644_to_0679;
mod section_0680_to_0698;
mod section_0699_to_0718;
mod section_0719_to_0767;
mod section_0768_to_0812;
mod section_0813_to_0861;
mod section_0862_to_0890;
mod section_0891_to_0899;
mod section_0900_to_0918;
mod section_0919_to_0941;
mod section_0942_to_0966;
mod section_0967_to_0979;
mod section_0980_to_1028;
mod section_1029_to_1054;
mod section_1055_to_1135;
mod section_1136_to_1198;
mod section_1199_to_1207;
mod section_1208_to_1298;
mod section_1299_to_1329;
mod section_1330;
mod section_1331;
mod section_1332;
mod section_1333;
mod section_1334;
mod section_1335;
mod section_1336;
mod section_1337;
mod section_1338_to_1339;
mod section_1340_to_1377;
mod section_1378;
mod section_1379;
mod section_1380;

pub use section_1332::entry;