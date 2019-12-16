fn calculate_fuel_requirement(input: f32) -> f32 {
    return (input / 3.0).floor() - 2.0;
}

fn calculate_part1(input: &str) {
    let total = input
        .split("\n")
        .map(|item| calculate_fuel_result_from_string(item))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum::<f32>();

    println!("total for day 1: {:?}", total);
}

fn calculate_fuel_needed(input: f32) -> f32 {
    let mut fuel = calculate_fuel_requirement(input);

    if fuel > 0.0 {
        fuel += calculate_fuel_needed(fuel);
    }

    return fuel.max(0.0);
}

fn calculate_part2(input: &str) {
    let values = input
        .split("\n")
        .map(|item| item.parse::<f32>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .map(calculate_fuel_needed)
        .sum::<f32>();

    println!("values: {:?}", values);

    println!("mass requirement for 14: {:?}", calculate_fuel_needed(14.0));
    println!("mass requirement for 1969: {:?}", calculate_fuel_needed(1969.0));
    // values is a list of floats
}

fn calculate_fuel_result_from_string(input: &str) -> Option<f32> {
    let value = input.parse::<f32>();

    if value.is_ok() {
        let value = (value.unwrap() / 3.0).floor() - 2.0;
        return Some(value);
    }

    return None;
}

fn main() {
    let content = "
50350
104487
101866
143582
58497
69981
98300
119291
148489
83005
107291
124738
142256
108102
121054
119697
75546
109022
136754
52073
115235
87668
64523
71179
69071
142380
68233
115226
132656
137007
82838
79339
131726
52295
102941
98297
144374
118998
63910
146772
82916
72068
82855
55915
91663
82917
105876
119551
70639
114459
129235
56041
70031
145187
54913
56928
52159
144384
80104
83932
81334
72693
50595
128895
54138
79126
69930
72896
108357
67415
110581
131477
65517
87912
125782
51785
145472
54358
87715
98067
99791
92502
50750
76614
110137
56118
149501
76542
87183
128333
127657
144246
141704
96873
62434
136609
121829
111796
103936
69807
";


    calculate_part1(content);
    calculate_part2(content);

}
