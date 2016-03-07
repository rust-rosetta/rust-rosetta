// http://rosettacode.org/wiki/The_Twelve_Days_of_Christmas

fn showpresents(count: usize) {
    let days = ["second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
                "tenth", "eleventh", "twelfth"];

    let presents = ["Two turtle doves",
                    "Three french hens",
                    "Four calling birds",
                    "Five golden rings",
                    "Six geese a-laying",
                    "Seven swans a-swimming",
                    "Eight maids a-milking",
                    "Nine ladies dancing",
                    "Ten lords a-leaping",
                    "Eleven pipers piping",
                    "Twelve drummers drumming"];

    println!("On the {} day of Christmas my true love gave to me {}",
             days[count - 1],
             presents[count - 1]);
    if count > 0 {
        let mut j = count - 1;
        while j > 0 {
            println!("{}", presents[j - 1]);
            j -= 1;

        }
    }
    println!("And a partridge in a pear tree \n");
}

fn main() {
    println!("On the first day of Christmas my true love gave to me a partridge in a pear tree\n");

    for count in 1..12 {
        showpresents(count);
    }
}
