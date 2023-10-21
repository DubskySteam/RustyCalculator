pub fn eval_math(sum: &f64, num: &str, op: &usize) -> f64 {
    match op {
        1 => println!("{} / {}", sum, num),
        2 => println!("{} * {}", sum, num),
        3 => println!("{} - {}", sum, num),
        4 => println!("{} + {}", sum, num),
        _ => println!("{} {} {}", sum, op, num)
    }

    let num = num.parse::<f64>().unwrap();
    match op {
        1 => *sum as f64/num,
        2 => *sum as f64 * num,
        3 => *sum as f64 - num,
        4 => *sum as f64 + num,
        _ => 0.0
    }
}
