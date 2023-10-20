// pub fn eval_math(sum: &f64, num: f64, op: &usize) -> f64 {
//     match op {
//         0 => num,
//         1 => sum/num,
//         2 => sum * num,
//         3 => sum - num,
//         4 => sum + num,
//         _ => 0.0
//     }
// }

//pub fn 

pub fn eval_math(sum: &f64, num: &str, op: &usize) -> f64 {
    println!("Calculating {} {} {}", sum, op, num);
    let num = num.parse::<f64>().unwrap();
    match op {
        0 => num,
        1 => *sum as f64/num,
        2 => *sum as f64 * num,
        3 => *sum as f64 - num,
        4 => *sum as f64 + num,
        _ => 0.0
    }
}
