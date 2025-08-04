fn main() {
    println!("Task 4: Control Flow");

    let number = 15;

    // TODO: Use if/else to check if 'number' is:
    // - Less than 10: print "Small number"
    // - Between 10 and 20: print "Medium number"
    // - Greater than 20: print "Large number"

    if number < 10 {
        println!("Small number");
    } else if number >= 10 && number <= 20 {
        println!("Medium number");
    } else {
        println!("Large number");
    }

    // TODO: Use a for loop to print numbers from 1 to 5
    for i in 1..6 {
        println!("{i}");
    }

    // TODO: Use a while loop to count down from 5 to 1
    let mut i = 5;
    while i > 0 {
        println!("{i}");
        i -= 1;
    }

}
