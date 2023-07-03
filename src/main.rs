use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut elfs: [i32; 242] = [0; 242];

    let mut elf_index = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line?;

        if line.trim().is_empty() {
            elf_index += 1;
            continue;
        }
        println!("Value: {}", line);
        elfs[elf_index] = line.parse::<i32>().unwrap() + elfs[elf_index];
    }
    println!("{}", elf_index);

    let mut fattest_elf = 0;
    for &elf in &elfs{
        if &elf > &fattest_elf {
            fattest_elf = *&elf;
        }
        println!("Value: {}", elf);
    }

    println!("fattest elf: {}", fattest_elf);

    Ok(())
}