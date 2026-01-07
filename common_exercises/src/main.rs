fn main() {
    println!("Hello, world!");
    let celcius = convert_to_celcius(68.0);
    println!("68 Fahrenheit is {celcius} Celsius");
    let fib = fibonacci(10);
    println!("The 10th Fibonacci number is {fib}");
    christmas_song();
}

fn convert_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn christmas_song() {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    for day in 1..12 {
        println!("On the {} day of Christmas my true love sent to me", day);
        for line in (0..day).rev() {
            println!("{}", gifts[line]);
        }
        println!("");
    }
}
