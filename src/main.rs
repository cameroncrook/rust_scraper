use reqwest;
use select::document::Document;
use select::document::Find;
use select::predicate::{Class, Element, Name, Predicate};

fn main() {
    let url = "https://realpython.github.io/fake-jobs/";

    let res = reqwest::blocking::get(url).unwrap();

    // Halts the program if res does not return as a success
    assert!(res.status().is_success());

    // Parse the HTML
    let document = Document::from_read(res).unwrap();

    for listing in document.find(Name("div").and(Class("card"))) {
        // listing data
        let mut listing_data: Vec<String> = Vec::new();

        // job title
        if let Some(job_title) = listing.find(Name("h2")).next() {
            listing_data.push(job_title.text());
        }

        // company
        if let Some(company) = listing.find(Name("h3")).next() {
            listing_data.push(company.text());
        }

        // location
        if let Some(location) = listing.find(Name("p").and(Class("location"))).next() {
            listing_data.push(location.text().trim().to_string());
        }

        // date
        if let Some(date) = listing.find(Name("time")).next() {
            listing_data.push(date.text());
        }

        if check_keywords(&listing_data[0]) {
            print_job_data(listing_data);
        }
    }
}

fn check_keywords(job_title: &str) -> bool {
    let keywords = ["Engineer", "Developer", "Python", "Software", "Data"];

    for word in keywords {
        if job_title.contains(word) {
            return true;
        }
    }

    return false;
}

fn print_job_data(listing_data: Vec<String>) {
    println!("");
    println!("Job Title: {}", listing_data[0]);
    println!("Company: {}", listing_data[1]);
    println!("Location: {}", listing_data[2]);
    println!("Date: {}", listing_data[3]);

    return;
}



// Practice
// -----------------------------------------

fn practice() {
    // Variables
    // ====================================
    // mut lets you change the value
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Print for debugging purposes
    let y = 6;
    let result = dbg!(y + 6);

    let currency = 12.50;
    println!("Balance: ${currency}");

    let word = "This is a string";
    println!("{word}");

    let fact_check = true;
    println!("{}", fact_check);



    

    // Conditionalls and expressions
    // =========================================
    if x == 2 && fact_check {
        println!("Both conditions were met");
    } else if x == 2 || fact_check {
        println!("Either x is 2 or fact_check is true");
    } else {
        println!("Neither conditions were met");
    }

    if x == 2 {
        println!("x is equal to 2");
    } else if fact_check {
        println!("fact_check is true");
    } else {
        println!("Neither is true");
    }

    let conditional_value = if fact_check {12} else {6};
    println!("{}", conditional_value);




    // Loops
    // ============================================
    let family = vec!["Cameron", "Gage", "Jayden", "Chase", "Tanner", "Alee", "Conner"];
    println!("");
    for person in family.iter() {
        println!("{}", person);
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    let mut num = 0;
    while num <= 5 {
        println!("Looped");

        num += 1;
    }
}
