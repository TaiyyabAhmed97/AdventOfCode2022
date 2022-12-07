use std::fs;
fn main() {
    let day_1_result = day_1();
    println!("{day_1_result}");
}

fn day_2() {
    
}

fn day_1() -> i32 {
    let contents = fs::read_to_string("/home/zahra/Projects/AdventOfCode2022/src/input.txt").expect("hello from here");
    let  splitted: Vec<&str> = contents.split("\n").collect();

    let mut index = 0;
    let mut global_max = 0;
    let mut local_max = 0;

    let mut maxes_vec = Vec::new();

    for calorie in splitted {
        if calorie.is_empty() {
            if local_max > global_max {
                global_max = local_max;
            }
            maxes_vec.push(local_max);
            local_max = 0;
            index += 1;
        } else {
            let parsed_int: i32 =  calorie.parse().unwrap();
            local_max += parsed_int;
        }
    }
    maxes_vec.sort();
    
    return maxes_vec[maxes_vec.len()-1] + maxes_vec[maxes_vec.len()-2] + maxes_vec[maxes_vec.len()-3];

}
