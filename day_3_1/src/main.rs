use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let values = content.split("\r\n").collect::<Vec<_>>();
    const N: usize = 12;
    const MAX: u32 = 2_u32.pow(N as u32) - 1;
    let mut v_count = vec![(0, 0); N];
    values.into_iter().for_each(|elem| {
        for (i, ch) in elem.chars().enumerate() {
            match ch {
                '0' => v_count[i].0 += 1,
                '1' => v_count[i].1 += 1,
                _ => panic!("Wrong symbol!"),
            }
        }
    });
    let mut num = 0;
    for i in 0..N {
        if v_count[N - 1 - i].1 > v_count[N - 1 - i].0 {
            num += 2_u32.pow(i as u32);
        }
    }
    println!("{}", num * (MAX - num));
    Ok(())
}
