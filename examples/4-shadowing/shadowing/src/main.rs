fn main() {
    let mut height = 190;
    height = height - 20;

    // Assigning a value to a variable via shadowing
    let result = if height < 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("Result: {}", result);

    let health = if height < 180 {"good"} else {"unknown"};
    println!("Health: {}", health);
    
    // shadowing to a different object type
    let health = if height < 180 {true} else {false};

}