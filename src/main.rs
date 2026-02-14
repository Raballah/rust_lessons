///Determine all girls aged 14 years or below and 15 years or above. 
//Count the number of girls aged 14 years nd below or 15 years and above. 

fn main() {
    let ages = vec![11, 12, 16, 15, 14, 13, 11, 13, 12, 16, 18, 17, 19, 16, 12, 14, 19];

    let mut young = Vec::new();
    let mut old = Vec::new();

    for age in ages {
        if age <= 14 {
            young.push(age);
        } else {
            old.push(age);
        }
    }

    println!("List of girls aged 14 or below: {:?}", young);
    println!("Number of girls aged 14 or below: {}", young.len());

    println!("List of girls aged 15 or above: {:?}", old);
    println!("Number of girls aged 15 or above: {}", old.len());
}