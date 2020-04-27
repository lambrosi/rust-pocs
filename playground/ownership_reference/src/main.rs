fn main() {
    let s1 = String::from("test");
    display(s1);
    // display(s1);  --> this line not compile because 'display' method take ownership on var 's1'
    //                   and DROP that in the end of the function, so 's1' doesn't exist in memory anymore 

    let s2 = String::from("test2");
    another_display(&s2);
    another_display(&s2);   // --> this line is ok because var 's2' wass passed by reference
                            //     so the function doesn't take ownership
}

fn display(text: String) {
    println!("{}", text);
}

fn another_display(text: &String) {
    println!("ANOTHER: {}", text);
}
