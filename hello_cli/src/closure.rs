fn main() {
    let min = 3;
    let nums = vec![1, 2, 3];
    let kept: Vec<&i32> = nums.iter().filter(|n| **n >= min).collect();
    println!("{:?}", kept);
}
