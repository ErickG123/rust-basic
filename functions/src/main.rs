fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");

    let plus = plus_one(y);
    println!("The plus one result is: {plus}");
}

fn plus_one(y: i32) -> i32 {
    y + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
