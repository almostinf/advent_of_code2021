use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let values = content.split("\r\n").collect::<Vec<_>>();
    const N: usize = 12;

    let mut vec_max = values.clone();
    let mut vec_0 = Vec::new();
    let mut vec_1 = Vec::new();
    for i in 0..N {
        if vec_max.len() == 1 {
            break;
        }
        vec_max
            .iter()
            .for_each(|elem| match elem.chars().nth(i).unwrap() {
                '0' => vec_0.push(elem.clone()),
                '1' => vec_1.push(elem.clone()),
                _ => panic!("Wrong symbol!"),
            });
        if vec_0.len() > vec_1.len() {
            vec_max = vec_0.clone();
            vec_0.clear();
            vec_1.clear();
        } else {
            vec_max = vec_1.clone();
            vec_0.clear();
            vec_1.clear();
        }
    }

    let mut vec_min = values;
    for i in 0..N {
        if vec_min.len() == 1 {
            break;
        }
        vec_min
            .iter()
            .for_each(|elem| match elem.chars().nth(i).unwrap() {
                '0' => vec_0.push(elem.clone()),
                '1' => vec_1.push(elem.clone()),
                _ => panic!("Wrong symbol!"),
            });
        if vec_0.len() <= vec_1.len() {
            vec_min = vec_0.clone();
            vec_0.clear();
            vec_1.clear();
        } else {
            vec_min = vec_1.clone();
            vec_0.clear();
            vec_1.clear();
        }
    }

    let max = i32::from_str_radix(vec_max[0], 2).expect("Not a number");
    let min = i32::from_str_radix(vec_min[0], 2).expect("Not a number");
    println!("{}", max * min);
    Ok(())
}
