use std::collections::VecDeque;
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
    let mut v = VecDeque::with_capacity(3);
    let count = values
        .into_iter()
        .filter(|elem| {
            if v.len() == 3 {
                v.pop_front();
            }
            v.push_back(*elem);
            if v.len() == 3 {
                if prev == 0 {
                    prev = v.iter().sum::<i32>();
                    false
                } else {
                    let res = v.iter().sum::<i32>() > prev;
                    prev = v.iter().sum::<i32>();
                    res
                }
            } else {
                false
            }
        })
        .count();
    println!("{}", count);
    Ok(())
}
