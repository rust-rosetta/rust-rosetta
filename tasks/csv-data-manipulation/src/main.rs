use std::vec::Vec ;

fn check_csv_length(values:&[Vec<String>],line_length:usize) -> Result<(),String>{
    match values.iter().all(|line| line.len() == line_length) {
        true => Ok(()),
        false => Err("Values have inconsistent number of columns".to_string())
    }
}

fn csv_sum(csv:&str) -> Result<Vec<Vec<String>>,String> {
    let mut lines : Vec<Vec<String>> = csv
        .split('\n') // split the string in lines
        // for each line
        .map(|line| line.split(',') // split the line into subparts
                        .map(|value| value.trim().to_string())
                        // convert every &str into an owned String
                        .collect::<Vec<String>>()) // transform the iterator into a vector
        .filter(|line| {
            match line.split_first() {
                None => unreachable!(), // even with an empty string, split will always return
                // a non-empty vector, hence here split here unreachable
                Some((string,slice)) if slice.is_empty()
                                     && string.trim().len() == 0 => false,  // filter empty lines
                _ => true // non-empty line, don't filter it
            }
        })
        .collect(); // transform the iterator over lines into a vector
    match lines.split_first_mut() {
        None => Err("The CSV file is empty !".to_string()) ,
        Some((_,ref values)) if values.is_empty() =>
            Err("The CSV file has no values !".to_string()),
        Some((ref mut headers,ref mut values)) => {
            let columns_number = headers.len();
            check_csv_length(values,columns_number).and_then(|_|{
                // if check_csv_length is already an error, return it,
                // otherwise do something with result (which in this case is '()', but we will
                // rather use 'headers' and 'values' for our stuff' )

                headers.push("SUM".to_string());
                // Since every line has the correct number of columns, add a "SUM" column

                let mut global_result = Ok(()) ;
                'lines: for (line_number,ref mut line) in values.iter_mut().enumerate() {
                    let line_number = line_number + 2 ;
                    // + 1 because enumerate is 0 indexed, and usually lines start from line 1;
                    // and another + 1 because this enumerate counts the line values,
                    // not all the lines. Note that it will not display the correct line if
                    // the file has some empty lines
                    let sum = {
                        // let's calculate the sum of a line
                        let f64_values = line.iter()
                            .map(|value| value.parse::<f64>() );
                            // map all the values of a line py parsing them as f64
                            // if it fails, it maps as an Err, otherwise an Ok( value )
                        let mut sum : Result<f64,_> = Ok(0.0);
                        'sum: for value in f64_values {
                            match value {
                                Ok(v) => {
                                    sum = sum.and_then(|f64_value|
                                         Ok(f64_value + v)
                                    );
                                },
                                Err(e) => {
                                    use std::error::Error;
                                    sum = Err(
                                        format!("Error \"{error}\" at line {line}",
                                                error = e.description(),
                                                line = line_number)
                                    );
                                    // a parsing error has been found
                                    break 'sum; // break the 'sum' loop, pointless to keep it going
                                }
                            };
                        };
                        sum
                    };
                    match sum {
                        Ok(value) => {
                            line.push(value.to_string());
                        },
                        Err(err) => {
                            global_result = Err(err);
                            break 'lines;
                        }
                    };
                };
                global_result
            })
        }
    }.and_then(|_|{
        Ok(lines)
    })
}

fn vec_csv_to_string(csv:&Vec<Vec<String>>) -> String {
    let mut string = String::new();
    for line in csv.iter() {
        string.push_str(&line.join(","));
        string.push('\n');
    };
    string
}

fn main(){
    let example_file : &'static str =
        "C1,C2,C3,C4,C5\n\
        1,5,9,13,17\n\
        2,6,10,14,18\n\
        3,7,11,15,19\n\
        4,8,12,16,20";
    println!("INPUT : ");
    println!("{}",example_file);
    println!("--------");
    println!("OUTPUT : ");
    match csv_sum(example_file) {
        Ok(csv) => {
            println!("{}",vec_csv_to_string(&csv));
        },
        Err(err_string) => {
            println!("An error occured : {desc}",desc = err_string);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{vec_csv_to_string,csv_sum};
    #[test]
    fn check_empty_csv(){
        let empty_file : &'static str = "" ;
        let no_values_file : &'static str = "C1,C2,C3,C4,C5\n";
        assert!(csv_sum(empty_file).is_err());
        assert!(csv_sum(no_values_file).is_err());
    }

    #[test]
    fn check_correct_csv(){
        let correct_file : &'static str =
            "C1,C2,C3,C4,C5\n\
            1,5,9,13,17\n\
            2,6,10,14,18\n\
            3,7,11,15,19\n\
            4,8,12,16,20";
        let whitespace_file : &'static str =
            "C1,C2,C3,C4,C5\n\
            1,5,9,13,17\n\
            2,6,10,14,18\n\
            \n\
            3,7,11,15,19\n\
            4,8,12,16,20";
        assert!(csv_sum(correct_file).is_ok());
        let csv = csv_sum(correct_file);
        assert!(csv.is_ok());
        assert_eq!(vec_csv_to_string(&csv.unwrap()),
        "C1,C2,C3,C4,C5,SUM\n\
        1,5,9,13,17,45\n\
        2,6,10,14,18,50\n\
        3,7,11,15,19,55\n\
        4,8,12,16,20,60\n".to_string());
        assert!(csv_sum(whitespace_file).is_ok());
    }

    #[test]
    fn check_incomplete_csv(){
        let incomplete_file : &'static str =
            "C1,C2,C3,C4,C5\n\
            1,5,9,13,17\n\
            2,6,18\n\
            3,7,11,15,19\n\
            4";
        assert!(csv_sum(incomplete_file).is_err());
    }

    #[test]
    fn check_wrong_csv(){
        let wrong_file : &'static str =
            "C1,C2,C3,C4,C5\n\
            1,5,9,13,17\n\
            2,6,error,14,18\n\
            3,7,11,15,19\n\
            4,8,12,16,20";
        assert!(csv_sum(wrong_file).is_err());
    }
}
