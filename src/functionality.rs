pub fn eval_math(sum: &usize, num: usize, op: &usize) -> usize {
    match op {
        0 => num,
        1 => sum/num,
        2 => sum * num,
        3 => sum + num,
        4 => sum - num,
        _ => 0
    }
}
