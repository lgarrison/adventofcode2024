use std::fs;

#[derive(Debug, Clone, Copy)]
struct Block {
    len: isize,
    id: isize,
    free: bool,
}

impl Block {
    fn split(&self, n: isize) -> [Block; 2] {
        [
            Block {
                len: n,
                id: self.id,
                free: self.free,
            },
            Block {
                len: self.len - n,
                id: self.id,
                free: self.free,
            },
        ]
    }
}

fn full_map(disk_map: &DiskMap) -> Vec<isize> {
    let mut res = vec![];
    for b in disk_map {
        for _ in 0..b.len {
            res.push(if b.free { 0 } else { b.id });
        }
    }
    res
}

fn print_full_map(disk_map: &DiskMap) {
    for b in disk_map {
        if b.free {
            print!(".")
        } else {
            print!("{}", b.id);
        }
    }
    print!("\n");
}

type DiskMap = Vec<Block>;

fn defrag(disk_map: &mut DiskMap) {
    let mut j = disk_map.len();

    while j > 0 {
        j -= 1;

        if disk_map[j].free {
            continue;
        }

        let mut i = 0usize;
        while i < j && (!disk_map[i].free || !(disk_map[i].len >= disk_map[j].len)) {
            i += 1;
        }

        if i >= j {
            continue;
        }

        assert!(disk_map[i].free);
        assert!(!disk_map[j].free);

        let b3 = disk_map.remove(j);

        // split the free space
        let [b1, b2] = disk_map[i].split(b3.len);

        disk_map[i] = b3;
        disk_map.insert(j, b1);

        if b2.len > 0 {
            disk_map.insert(i + 1, b2);
            j += 1;
        }
        // disk_map.swap(i, j);
    }
}

fn checksum(disk_map: &DiskMap) -> isize {
    full_map(&disk_map)
        .iter()
        .enumerate()
        .map(|(i, id)| i as isize * id)
        .sum()
}

fn part1(txt: &str) -> isize {
    let mut disk_map: DiskMap = txt
        .char_indices()
        .flat_map(|(i, c)| {
            [Block {
                len: 1,
                id: (i / 2) as isize,
                free: i % 2 == 1,
            }]
            .repeat(c.to_digit(10).unwrap() as usize)
        })
        .collect();
    defrag(&mut disk_map);
    checksum(&disk_map)
}

fn part2(txt: &str) -> isize {
    let mut disk_map: DiskMap = txt
        .char_indices()
        .map(|(i, c)| Block {
            len: c.to_digit(10).unwrap() as isize,
            id: (i / 2) as isize,
            free: i % 2 == 1,
        })
        .collect();
    defrag(&mut disk_map);
    checksum(&disk_map)
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test2.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    // println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}
