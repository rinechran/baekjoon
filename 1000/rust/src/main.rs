use std::io;


fn main() {
    let mut nums_string = String::new();
    io::stdin().read_line(&mut nums_string).expect("");
    let nums : Vec<u32> = nums_string.split_whitespace()
        .map(|s| s.trim().parse().expect(""))
        .collect::<Vec<_>>();

    let result : u32 = nums[0] + nums[1];
    print!("{}",result);

}
