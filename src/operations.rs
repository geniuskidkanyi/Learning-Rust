use std::f64::consts;

fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main(){
    let pi: f64 = 3.1416;
    let x = pi/2.0;
    let cosine = x.cos();
    println!("Cosing is {}", cosine);

    let x = 2.0 * consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);

    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("First {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
    }

    println!("length {}", arr.len());

    let arr = [10, 20, 30, 40, 50, 60, 70, 80];
    let res = sum(&arr);
    println!("Sum {}", res);

}