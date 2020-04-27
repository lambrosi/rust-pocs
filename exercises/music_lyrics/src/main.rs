fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let gifts = ["A partridge in a pear tree",
            "Two turtle doves",
            "Three French hens", 
            "Four calling birds",
            "Five gold rings",
            "Six geese a laying",
            "Seven swans a swimming",
            "Eight maids a milking",
            "Nine ladies dancing",
            "Ten lords a leaping",
            "Eleven pipers piping",
            "Twelve drummers drumming"];
            
    for day_pos in 0..days.len() {
        println!("On the {} day of Christmas my true love gave to me", days[day_pos]);
    
        for gift_pos in (0..=day_pos).rev() {
            if gift_pos == 0 {
                println!("And {}", gifts[gift_pos]);
            } else {
                println!("{}{}", gifts[gift_pos], define_appender(day_pos));
            }
        }
        println!();
    }
}

fn define_appender(day_pos: usize) -> String {
    if day_pos > 0 {
        return String::from(",")
    }
    return String::new()
}
