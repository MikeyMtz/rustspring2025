fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 25, 30, 7, 19, 10, 15, 42, 8, 5];
    
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            println!("{}: {}", num, if is_even(num) { "Even" } else { "Odd" });
        }
    }
    
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);
    
    let mut max_num = numbers[0];
    for &num in &numbers {
        if num > max_num {
            max_num = num;
        }
    }
    println!("Largest number: {}", max_num);
}
