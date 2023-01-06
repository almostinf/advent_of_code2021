use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let values = content
        .split("\r\n")
        .map(|elem| {
            elem.split(' ')
                .collect_tuple::<(&str, &str)>()
                .expect("Can't convert to tuple")
        })
        .collect::<Vec<_>>();
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    values.into_iter().for_each(|elem| match elem.0 {
        "forward" => {
            let x = elem.1.parse::<i32>().expect("Can't convert to i32");
            forward += x;
            depth += x * aim;
        }
        "down" => aim += elem.1.parse::<i32>().expect("Can't convert to i32"),
        "up" => aim -= elem.1.parse::<i32>().expect("Can't convert to i32"),
        _ => panic!(),
    });
    println!("{}", depth * forward);
    Ok(())
}
