use std::fs;
fn main() {
    let contents = fs::read_to_string("/home/zahra/Projects/hello-world/src/input.txt").expect("hello from here");
    let  splitted: Vec<&str> = contents.split("\n").collect();

    let mut index = 0;
    let mut global_max = 0;
    let mut second_place_max = 0;
    let mut third_place_max = 0;
    let mut max_index = 0;
    let mut local_max = 0;

    let mut maxes_vec = Vec::new();

    for calorie in splitted {
        if calorie.is_empty() {
            println!("hello");
            if local_max > global_max {
                global_max = local_max;
                max_index = index;
                println!("global is changed to {global_max} at {index}, was compared to {local_max}");
            }
            maxes_vec.push(local_max);
            local_max = 0;
            index += 1;
        } else {
            let parsed_int: i32 =  calorie.parse().unwrap();
            local_max += parsed_int;
            println!("{local_max} at {index}");
        }
    }
    maxes_vec.sort();
    println!("{:?}", maxes_vec[maxes_vec.len()-1] + maxes_vec[maxes_vec.len()-2] + maxes_vec[maxes_vec.len()-3]);
    
    println!("Hello, world!");
}
