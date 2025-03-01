use core::cmp::max;
use core::cmp::min;
use std::collections::HashMap;

type Coord = (i32, i32);
type TwoCoords = (Coord, Coord);

fn is_horizontal(c: &TwoCoords) -> bool {
    return (c.0).1 == (c.1).1;
}

fn do_path(path: Vec<&str>) -> Vec<TwoCoords> {
    let mut coords: Vec<TwoCoords> = Vec::with_capacity(path.len() + 1);
    let mut current = (0, 0);

    for p in &path {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let direction = p.chars().next().unwrap();
        let num = &p[1..].parse::<i32>().unwrap();

        match direction {
            'R' => x += num,
            'L' => x -= num,
            'U' => y += num,
            'D' => y -= num,
            _ => println!("unknown direction!"),
        }

        let first = current.clone();

        current.0 += x;
        current.1 += y;

        coords.push((first, current));
    }

    return coords;
}

fn get_path_range(line: &TwoCoords) -> TwoCoords {
    return (
        (min((line.0).0, (line.1).0), min((line.0).1, (line.1).1)),
        (max((line.0).0, (line.1).0), max((line.0).1, (line.1).1)),
    );
}

fn get_path_intersection(line1: &TwoCoords, line2: &TwoCoords) -> Option<Coord> {
    let line1 = get_path_range(&line1);
    let line2 = get_path_range(&line2);

    if is_horizontal(&line1) && is_horizontal(&line2) {
        return None;
    }

    if !is_horizontal(&line1) && !is_horizontal(&line2) {
        return None;
    }

    if is_horizontal(&line1) {
        let x = (line2.0).0;
        let y = (line1.0).1;

        if (line1.0).0 <= x && (line1.1).0 >= x && (line2.0).1 <= y && (line2.1).1 >= y {
            return Some((x, y));
        }
    } else {
        let x = (line1.0).0;
        let y = (line2.0).1;

        if (line2.0).0 <= x && (line2.1).0 >= x && (line1.0).1 <= y && (line1.1).1 >= y {
            return Some(((line1.0).0, (line2.0).1));
        }
    }

    None
}

fn path_collides(line: &TwoCoords, path: &Vec<TwoCoords>) -> Vec<Coord> {
    let coords: Vec<Coord> = path
        .iter()
        .map(|p| get_path_intersection(line, &p))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect();

    return coords;
}

fn part1(paths: &Vec<&str>) -> Vec<Coord> {
    let mut processed_paths: Vec<Vec<TwoCoords>> = Vec::new();

    for path in paths {
        let items = path.split(",").collect();
        let result = do_path(items);

        processed_paths.push(result);
    }

    let mut results: Vec<Coord> = Vec::new();

    for path in &processed_paths {
        for other in &processed_paths {
            if other == path {
                continue;
            }

            for line in path {
                let matches = path_collides(line, other);

                for m in matches {
                    results.push(m);
                }
            }
        }
    }

    let distances: Vec<i32> = results
        .iter()
        .map(|i| (i.0).abs() + (i.1).abs())
        .filter(|i| i > &0)
        .collect();

    let min_value = distances.iter().min();
    println!("answer1: {:?}", min_value.unwrap());

    return results;
}

fn is_between(num: i32, start: i32, end: i32) -> bool {
    // start is greater than end, so number should be smaller than start
    if start > end {
        return num <= start && num >= end;
    } else {
        return num >= start && num <= end;
    }
}
fn distance_until(path: &Vec<TwoCoords>, target: &Coord) -> i32 {
    let mut distance = 0;

    for l in path {
        let s = l.0;
        let e = l.1;
        let sx = s.0;
        let sy = s.1;
        let ex = e.0;
        let ey = e.1;
        // println!("{:?}", l);
        
        let dxy = (ex - sx).abs() + (ey - sy).abs();
        distance += dxy;

        // println!("adding {} to distance ({:?}) -> {:?}", dxy, l, target);

        if is_horizontal(&l) && target.1 == sy && is_between(target.0, sx, ex) {
            let rx = (ex - target.0).abs();
            // println!("is a match: {:?} and {:?}, {}", l, target, rx);
            distance -= rx;
            break;
        } else if !is_horizontal(&l) && target.0 == sx && is_between(target.1, sy, ey) {
            let rx = (ey - target.1).abs();
            // println!("is a match: {:?} and {:?}, {}", l, target, rx);
            distance -= rx;
            break;
        }
    }

    // println!("distance: {}", distance);

    return distance;
}

fn part2(paths: &Vec<&str>, intersections: &Vec<Coord>) {
    let path1 = do_path(paths[0].split(",").collect());
    let path2 = do_path(paths[1].split(",").collect());

    println!("path1: {:?}", path1);
    
    let mut distances: Vec<i32> = intersections.iter().filter(|c| c.0 != 0 && c.1 != 0)
        .map(|x| distance_until(&path1, x) + distance_until(&path2, x))
        .collect();
    
    distances.sort();
    println!("distances: {:?}", distances);
    println!("answer2: {}", distances[0]);
}

fn main() {
    let paths = vec![
        "R991,U847,L239,U883,L224,D359,L907,D944,L79,U265,L107,D183,R850,U203,R828,D95,L258,D931,R792,U117,L309,U182,L633,D567,L828,D454,L660,U652,L887,D341,L497,D857,L299,U191,L882,D476,L968,U913,R453,D776,R169,D1,L193,D187,L564,U306,R815,U9,L434,U879,L816,D142,R16,U663,L54,D347,L557,U828,R597,D328,L636,U200,L383,D256,R162,U159,R37,D748,R440,D260,R48,D755,R762,U73,L357,U132,L745,D426,L797,U744,R945,D788,R585,U948,L20,D983,L335,U709,R488,U715,R229,D672,L13,D930,R903,D71,R620,U146,L835,U936,R542,D311,R375,U91,R362,U613,L78,D451,R220,D493,R404,D516,L550,U647,L908,U254,R827,D180,R902,U972,R56,U761,R912,U356,L921,D461,L65,D651,L230,U534,R143,D614,L526,D100,R76,D135,L572,U971,L219,D793,R638,U676,L58,D882,R299,D922,L198,D872,R736,D433,L999,U157,R795,U344,R213,D205,L928,D319,L775,U288,L903,U735,R128,D835,R496,U992,L875,D823,L833,D635,L700,U586,L587,U753,R849,U433,R473,U369,R891,U10,L152,U26,L893,U752,L258,D384,L491,U314,R722,U783,R801,U551,R141,U870,L662,D572,R671,U285,L435,D83,L260,U371,R849,U741,R661,U774,L583,U947,L460,U677,R809,D130,L288,D58,R107,U597,R21,U17,R99,U202,L324,U493,R824,U207,L460,D734,L154,D689,L366,D879,L353,U548,L307,D691,R70,U470,R649,D948,L346,U16,L257,D800,R954,D165,R376,D312,R491,D175,R426,U920,L532,U2,L556,D553,R320,D861,L129,D42,R112,U101,R455,D930,R122,D443,R28,D72,L670,U133,L599,D813,R169,D827,R235,D644,L297,U261,R405,D887,R218,D647,R108,D928,L779,D961,L110,U690,L214,U342,R449,D737,L651,U940,L370,D882,R10,D605,R369,U408,R167,D542,L819", "L994,U274,R468,D607,R236,D712,R825,D228,L812,U796,R806,D874,L742,D297,L269,D853,R229,U319,R616,U77,L30,D879,L831,U241,R751,D20,R577,D949,L333,D520,L249,D165,R831,U965,L229,D412,L312,U31,L624,U593,L508,D359,R187,D682,R536,D266,L761,U412,R136,D296,L334,D180,R683,U93,L323,D864,L912,U262,L150,U437,L961,U224,R684,D62,R733,U302,R700,D417,R861,U394,L647,D564,R588,U184,L344,D812,L412,U409,R853,D548,L401,D670,R973,U490,R791,D784,R569,U852,R753,U510,R394,D517,R253,D418,R665,D742,L233,D311,L266,D395,L23,U595,R248,D243,L944,U830,L846,U44,L231,D399,R131,D825,R975,U476,L306,U716,L764,D730,L455,U27,L764,D274,R403,D376,L474,D724,R237,U870,R206,U172,R857,D993,R348,U591,R228,U534,L968,U722,L891,U656,L645,U831,L838,D641,R886,U185,R760,U531,R397,D849,L790,U839,L937,U508,L802,U166,L571,D153,L600,U356,R273,D185,L862,D159,L806,U503,R612,U324,R745,D398,L905,D31,L14,U965,R586,U808,L334,U390,R44,D132,R605,U999,R880,U579,R732,D717,L489,D577,R373,D913,R238,U532,R614,U518,R197,U129,R627,U5,R774,D922,L761,D540,R418,U419,R120,U637,R237,D73,L648,D162,L324,D911,L916,D886,L60,D961,R207,U102,R872,D884,R611,U360,R679,U974,R30,U895,L327,U256,L520,U977,R792,D356,R376,D39,L689,U159,R270,D621,L197,U138,L811,U100,L776,U936,R514,D69,R625,U99,L970,D519,R831,U227,L307,D271,R940,U690,L978,D257,R500,D971,R149,U291,L706,U177,L694,U230,R780,U604,R987,U222,L941,D511,R591,U156,L511,U207,L423,U324,R508,U338,L257,U547,R952,U927,L205,U476,L713,D170,L462,D848,R666,D836,R352,U414,L653,D657,R721,U807,L182,U823,L826"
    ];

    let results = part1(&paths);
    part2(&paths, &results);
}
