use std::io::stdin;

struct File {
    id: usize,
    offset: usize,
    size: usize,
}

fn main() {
    let mut files = parse_input(stdin().lines().map(|l| l.unwrap()).nth(0).unwrap().as_str());

    // part 1
    let mut disk = materialize(&files);
    rearrange_blocks(&mut disk);
    println!("{}", checksum(&disk));

    // part 2
    rearrange_files(&mut files);
    let disk = materialize(&files);
    println!("{}", checksum(&disk));
}

fn parse_input(i: &str) -> Vec<File> {
    let mut id = 0;
    let mut offset = 0;
    let mut files = Vec::new();

    for (index, char) in i.trim().char_indices() {
        let size = char.to_digit(10).unwrap() as usize;

        if index % 2 == 0 {
            files.push(File { id, offset, size });
            id += 1;
        }

        offset += size;
    }

    files
}

fn materialize(files: &[File]) -> Vec<Option<usize>> {
    let mut disk = vec![None; files.last().map_or(0, |f| f.offset + f.size)];

    for file in files {
        for i in file.offset..file.offset + file.size {
            disk[i] = Some(file.id);
        }
    }

    disk
}

fn rearrange_blocks(disk: &mut [Option<usize>]) {
    let mut from = disk.len() - 1;
    let mut to = 0;

    loop {
        while disk[from].is_none() {
            from -= 1;
        }

        while disk[to].is_some() {
            to += 1;
        }

        if from <= to {
            break;
        }

        disk.swap(from, to);
    }
}

fn rearrange_files(files: &mut [File]) {
    let mut from = files.len() - 1;

    'outer: while from > 0 {
        for to in 0..from {
            let offset = files[to].offset + files[to].size;

            if files[to + 1].offset - offset >= files[from].size {
                files[from].offset = offset;
                files[to + 1..=from].rotate_right(1);
                continue 'outer;
            }
        }

        from -= 1;
    }
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, id)| id.map(|id| i * id))
        .sum()
}
