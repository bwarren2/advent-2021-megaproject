fn part1(values: &Vec<i32>) -> i32 {
    let minimum_value = *values.iter().min().unwrap();
    let maximum_value = *values.iter().max().unwrap();

    let mut usage: Vec<i32> = Vec::new();
    for x in minimum_value..maximum_value {
        let mut ct = 0;
        for value in values {
            if value > &x {
                ct = value - x + ct;
            } else {
                ct = x - value + ct;
            }
        }
        usage.push(ct);
    }
    *usage.iter().min().unwrap()
}

fn part2(values: &Vec<i32>) -> i32 {
    let minimum_value = *values.iter().min().unwrap();
    let maximum_value = *values.iter().max().unwrap();

    let mut usage: Vec<i32> = Vec::new();
    for x in minimum_value..maximum_value {
        let mut ct = 0;
        for value in values {
            if value > &x {
                ct = sum_usage(value - x) + ct;
            } else {
                ct = sum_usage(x - value) + ct;
            }
        }
        usage.push(ct);
    }
    *usage.iter().min().unwrap()
}


fn main() {
    let string = "1101,1,29,67,1102,0,1,65,1008,65,35,66,1005,66,28,1,67,65,20,4,0,1001,65,1,65,1106,0,8,99,35,67,101,99,105,32,110,39,101,115,116,32,112,97,115,32,117,110,101,32,105,110,116,99,111,100,101,32,112,114,111,103,114,97,109,10,51,13,782,658,1369,575,693,395,898,552,57,374,155,541,279,428,338,390,0,181,952,350,123,57,483,449,715,672,619,910,254,308,104,682,560,928,406,4,1328,878,36,1397,1111,1586,548,106,284,20,462,1248,1033,229,108,336,888,289,464,757,19,58,1,262,1202,576,101,928,265,781,145,829,1183,1220,181,194,1181,11,792,1542,1330,728,273,493,1753,327,1736,33,1547,750,1623,374,1603,26,116,169,1270,191,11,47,312,136,46,486,1478,28,64,258,74,351,85,105,1137,531,1573,56,676,532,141,955,98,4,541,1546,1771,432,844,228,103,281,404,2,1431,865,1094,12,134,483,152,308,135,135,257,439,5,174,906,148,1000,160,304,190,478,199,395,693,257,84,148,140,76,354,595,1,633,284,24,83,782,1359,1187,351,290,704,23,692,952,1417,192,868,349,56,936,279,277,115,1579,476,261,50,46,1144,568,11,495,170,726,888,548,509,1265,179,93,834,180,143,866,899,292,958,126,1608,1066,327,1149,74,227,346,110,736,592,989,27,12,123,325,650,49,76,0,299,85,261,729,1135,46,479,879,226,1146,781,852,426,184,274,1323,449,419,90,758,378,134,219,333,833,55,59,358,175,293,36,100,27,764,564,188,814,612,299,1057,829,322,235,122,330,280,1397,654,370,320,770,1082,92,1513,1085,563,455,322,664,14,1122,40,381,750,145,381,422,553,227,20,503,368,43,316,71,479,438,222,204,355,67,264,882,1168,416,14,74,371,293,3,1005,237,422,570,1391,818,451,987,529,79,28,664,454,820,1425,50,49,32,486,397,680,1247,207,6,742,84,1362,540,233,1082,847,28,199,84,111,858,362,23,1755,61,144,652,82,1379,372,329,900,255,850,1006,21,1345,252,938,214,394,12,333,354,834,298,132,246,167,140,423,50,35,463,1502,390,672,2,123,78,588,1202,523,1507,64,68,130,511,263,734,112,1463,1097,99,552,53,1189,343,496,16,260,225,1065,855,12,24,88,958,848,173,1131,1639,559,344,216,19,872,287,288,911,775,1722,1049,17,223,375,378,1109,977,520,350,138,376,75,638,284,167,328,467,146,1379,947,149,890,661,56,1339,148,346,693,52,812,963,198,120,1194,231,7,1558,148,545,325,1038,57,52,936,117,531,684,266,687,394,851,1068,571,627,558,172,998,909,1403,358,13,419,0,1247,81,46,647,833,1370,340,287,74,51,691,198,129,257,769,108,163,480,295,150,1944,1897,161,1081,1521,733,351,376,126,191,94,228,76,957,652,487,805,10,347,217,394,1142,725,780,50,234,654,233,119,1035,226,110,757,75,6,13,649,396,13,191,115,368,60,731,485,621,1454,669,518,29,41,495,768,504,209,222,1168,34,1061,1654,973,979,55,802,139,489,386,757,1258,1720,757,994,172,572,266,15,267,830,1105,30,112,188,26,54,205,1206,1193,688,8,1149,1355,983,1073,434,62,1242,264,181,146,8,20,968,418,607,105,22,221,392,127,388,1055,98,241,58,805,2,801,925,241,931,1184,472,563,167,398,388,1290,111,237,1167,332,293,166,51,319,201,40,128,58,90,270,525,1171,1222,1129,1657,525,427,69,340,547,1486,174,383,900,763,160,193,696,896,477,564,558,870,294,868,951,267,178,454,35,1042,627,1451,740,2,107,4,632,114,754,83,244,43,699,83,1001,226,404,956,522,73,505,193,662,548,525,110,1241,415,794,1954,1062,1217,32,98,4,355,143,146,900,369,1418,250,404,703,469,34,326,62,231,564,44,655,511,1139,1010,1438,354,1117,591,118,509,496,1438,232,1179,526,259,984,643,169,165,50,1136,86,702,412,1762,62,219,1089,687,10,29,330,893,1658,732,48,806,1158,854,14,44,548,627,811,41,1212,341,474,943,25,699,204,1449,769,134,23,28,1216,622,1371,366,233,8,29,615,16,166,256,324,169,657,21,854,674,10,1217,1499,1512,88,369,771,60,317,28,1251,14,1318,376,264,55,658,1873,1280,581,850,147,267,1147,150,6,18,1166,331,163,281,773,1284,501,588,1055,681,41,506,270,104,1212,440,181,1177,257,1444,406,111,662,75,28,214,892,349,380,1677,1885,88,398,713,1596,99,541,219,149,1222,455,29,78,217,618,33,94,335,1050,848,958,67,12,61,9,1133,96,151,1057,60,167,324,99,49,1159,571,270,1291,217,276,42,8,20,652,949,134,132,552,365,112,355,952,167,33,1053,994,445,277,652,569,1594,248";
    let values: Vec<i32> = string.split(",").map(|n| n.parse().unwrap()).collect();

    println!("{:?}",part1(&values));
    println!("{:?}",part2(&values));
}

fn sum_usage(units: i32) -> i32 {
    (units * units + units) / 2
}
