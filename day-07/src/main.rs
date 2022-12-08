use anyhow::Result;
use std::{collections::HashMap, fmt::Debug, str::Lines};
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-07/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();

    // Get rid of `$ cd /`
    lines.next();

    let tree = traverse(
        &mut lines,
        Folder {
            folders: HashMap::new(),
            files: HashMap::new(),
        },
    );

    let unused_size = 70000000 - tree.size();

    part_2_walk(&tree, unused_size)
}

fn part_2_walk(root: &Folder, unused_size: usize) -> usize {
    let mut smallest = root.size();

    for folder in root.folders.values() {
        let folder_size = part_2_walk(folder, unused_size);

        if unused_size + folder_size >= 30000000 && folder_size < smallest {
            smallest = folder_size
        }
    }

    smallest
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();

    // Get rid of `$ cd /`
    lines.next();

    let tree = traverse(
        &mut lines,
        Folder {
            folders: HashMap::new(),
            files: HashMap::new(),
        },
    );

    part_1_walk(&tree)
}

fn part_1_walk(root: &Folder) -> usize {
    let mut sum = 0;

    for folder in root.folders.values() {
        let x = folder.size();
        if x <= 100000 {
            sum += x;
        }

        sum += part_1_walk(folder);
    }

    sum
}

fn traverse(lines: &mut Lines, mut root: Folder) -> Folder {
    loop {
        let Some(current) = lines.next() else {
            break
        };

        if current.starts_with("$ ls") {
            let mut consumed_n = 0;

            for line in lines.clone() {
                consumed_n += 1;
                if line.starts_with('$') {
                    break;
                } else if line.starts_with("dir ") {
                    let name = line.split(' ').nth(1).unwrap().to_string();

                    root.folders.insert(
                        name.clone(),
                        Folder {
                            folders: HashMap::new(),
                            files: HashMap::new(),
                        },
                    );
                } else {
                    let mut sepr = line.split(' ');
                    let (size, name) = (
                        sepr.next().unwrap().parse::<usize>().unwrap(),
                        sepr.next().unwrap().to_string(),
                    );

                    root.files.insert(name.clone(), File { size });
                }
            }

            for _ in 0..consumed_n - 1 {
                lines.next();
            }
        } else if current.starts_with("$ cd ..") {
            break;
        } else if current.starts_with("$ cd") {
            let name = current[5..].to_string();
            let dir = traverse(lines, root.folders.get(&name).unwrap().clone());

            root.folders.insert(name, dir);
        } else {
            break;
        }
    }

    root
}

#[derive(Clone, Debug)]
struct Folder {
    pub folders: HashMap<String, Folder>,
    pub files: HashMap<String, File>,
}

impl Folder {
    fn size(&self) -> usize {
        self.files.iter().map(|(_, file)| file.size).sum::<usize>()
            + self
                .folders
                .iter()
                .map(|(_, folder)| folder.size())
                .sum::<usize>()
    }
}

#[derive(Clone, Debug)]
struct File {
    pub size: usize,
}
