fn main() {
    let content = include_str!("../test.txt");
    let mut crabs: Vec<i32> = content
        .split(',')
        .map(|elem| elem.parse().expect("Expected a number"))
        .collect();
    crabs.sort();
    let mediana = if crabs.len() % 2 == 0 {
        (crabs[crabs.len() / 2 - 1] + crabs[crabs.len() / 2]) / 2
    } else {
        crabs[crabs.len() / 2]
    };
    let mut distation = 0;
    crabs
        .iter()
        .for_each(|elem| distation += (mediana - *elem).abs());
    println!("{}", distation);
}
