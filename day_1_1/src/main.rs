use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let values = content
        .split("\r\n")
        .map(|elem| elem.parse::<i32>().expect("Can't parse str"))
        .collect::<Vec<_>>();
    let mut prev = i32::default();
    let mut check = false;
    let count = values
        .into_iter()
        .filter(|elem| {
            if !check {
                prev = *elem;
                check = true;
                false
            } else {
                let res = *elem > prev;
                prev = *elem;
                res
            }
        })
        .count();
    println!("{}", count);
    Ok(())
}
