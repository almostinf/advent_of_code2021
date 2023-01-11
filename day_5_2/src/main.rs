use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let coordinates = content
        .split("\r\n")
        .map(|elem| {
            elem.split(" -> ")
                .map(|e| {
                    e.split(',')
                        .map(|e| e.parse::<i32>().expect("Not a number"))
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .collect_tuple::<((i32, i32), (i32, i32))>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut count_dots = HashMap::<(i32, i32), i32>::new();
    coordinates.into_iter().for_each(|(start, end)| {
        let len_x = end.0 - start.0;
        let len_y = end.1 - start.1;
        let dx = if len_x == 0 {
            0
        } else if len_x < 0 {
            -1
        } else {
            1
        };
        let dy = if len_y == 0 {
            0
        } else if len_y < 0 {
            -1
        } else {
            1
        };
        let mut new_x = start.0;
        let mut new_y = start.1;
        while new_x != end.0 || new_y != end.1 {
            count_dots
                .entry((new_x, new_y))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            new_x += dx;
            new_y += dy;
        }
        count_dots
            .entry((new_x, new_y))
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });
    println!("{}", count_dots.values().filter(|&elem| *elem >= 2).count());
    Ok(())
}
