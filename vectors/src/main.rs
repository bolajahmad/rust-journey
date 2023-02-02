use std::collections::HashMap;

fn get_median_and_mode(mut list: Vec<u32>) {
    // Given a list of integers
    // return the median and the mode
    list.sort();

    print!("This is the sorted list, {:?}", list);
    // Median will be the integer at the middle of the array.
    // if the index is odd, then the value is just that
    // if it is even, the answer is the average of the 2 numbers
    let list_length = list.len();
    let median: u32;
    if list_length % 2 == 1 {
        let mid = ((list_length.clone() as f64) / 2.0).floor().clone() as usize;
        median = list[mid];
    } else {
        let mid = list_length / 2;
        median = (list[mid] + list[mid - 1]) / 2;
    }

    print!("\n The deduced median is {median}");

    let mut mapped_list: HashMap<u32, u32> = HashMap::new();
    for i in 1..list_length {
        let count = mapped_list.entry(list[i - 1]).or_insert(0);
        *count += 1;
    }
    let mode = mapped_list
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k);
    println!("\n Mode of list is, {:?}", mode);
}

fn main() {
    let list = [2, 4, 7, 2, 2, 7, 10, 3, 8, 4, 11, 0];
    get_median_and_mode(list.to_vec());
    /*
       Convert strings to pig latin. The first consonant of each word is moved to the end
       of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
       a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
       Keep in mind the details about UTF-8 encoding!
    */
    let plain_text = String::from("Strength is Optional!!");
}
