mod operations;

fn sqr(x: f64) -> f64 {
         x * x
}
fn main() {
    let name ="muhammed kanyi";
    for i in 1..10 {
        println!("hello {}", i);
    };
    
    for i in 1..100  {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i)
        }

    }

    for i in 0..10  {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }

    let mut sum = 0;
    for i in 0..5 {
        sum += i ;
    }
    println!("sum is {}", sum);

    let res = sqr(2.0);
    println!("square is {}", res);

}
