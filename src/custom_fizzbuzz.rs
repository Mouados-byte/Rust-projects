fn main() {
    println!("Hello world!");
    fizzbuzz(10);
    
    println!("");
    let rules = vec![
        (3, "Fizz".to_string()),
        (5, "Buzz".to_string()),
        (15, "Bazz".to_string()),
    ];
    custom_fizzbuzz(40, &rules);
}

fn fizzbuzz(count: u8) {
    if count <= 0 {
        return;
    }
    for i in 0..=count{
        if i % 5 == 0 && i % 3 == 0 {
            println!("{} => {}", i, "FizzBuzz");
            continue;
        }
        if i % 3 == 0 {
            println!("{} => {}", i, "Fizz");
            continue;
        }
        if i % 5 == 0 {
            println!("{} => {}", i, "Buzz");
            continue;
        }
    }
}

fn custom_fizzbuzz(count: u8, rules: &[(u8, String)]){
    if count <= 0 {
        return;
    }
    for i in 0..=count{
        let mut s = String::new();
        let mut matches = false;
        for rule in rules{
            if i % rule.0 == 0 {
                s += &rule.1;
                matches = true;
            }
        }
        if matches {
            println!("{} => {}", i, s);
        }
    }
}