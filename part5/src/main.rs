fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    let (numbers, sum_of_nums) = sum(numbers);
    let (numbers, average_of_nums) = average(numbers);
    let (_, product_of_nums) = product(numbers);

    // 💡 TIP: You'll get a compile error. Here are two ways you can fix it:
    // Option 1: Pass numbers.clone() some of the time.
    //           (Experiment to see when it's needed!)
    //
    // Option 2: Change some of the functions to return a tuple
    //           of (i64, Vec<i64>), using the `numbers` argument
    //           as the Vec<i64> to return. With this approach,
    //           you won't need to call .clone() at all!
    //
    // Give both options a try!

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);
}

fn sum(numbers: Vec<i64>) -> (Vec<i64>, i64) {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    (numbers, total)
}

fn product(numbers: Vec<i64>) -> (Vec<i64>, i64) {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    (numbers, total)
}

fn average(numbers: Vec<i64>) -> (Vec<i64>, i64) {
    let length = numbers.len() as i64;
    let (numbers, sum) = sum(numbers);

    (numbers, sum / length)
}
