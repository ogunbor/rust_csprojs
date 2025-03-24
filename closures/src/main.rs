fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Filter even numbers using a closure
    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();

    println!("{:?}", evens); // Output: [2, 4]
}
