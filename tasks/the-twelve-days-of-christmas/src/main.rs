fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth",
                "ninth", "tenth", "eleventh", "twelfth"];

    let gifts = ["A Patridge in a Pear Tree",
                 "Two Turtle Doves and",
                 "Three French Hens",
                 "Four Calling Birds",
                 "Five Golden Rings",
                 "Six Geese a Laying",
                 "Seven Swans a Swimming",
                 "Eight Maids a Milking",
                 "Nine Ladies Dancing",
                 "Ten Lords a Leaping",
                 "Eleven Pipers Piping",
                 "Twelve Drummers Drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas,", days[i]);
        println!("My true love gave to me:");

        for j in (0..i + 1).rev() {
            println!("{}", gifts[j]);
        }
        println!()
    }
}
