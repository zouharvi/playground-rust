use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Cave {
    name: String,
    big: bool,
    neighbours: HashSet<String>,
    visited: bool,
}

impl Cave {
    fn new(name: String) -> Cave {
        Cave {
            big: name.chars().all(|c| c.is_ascii_uppercase()),
            name: name,
            neighbours: HashSet::<String>::new(),
            visited: false,
        }
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cave")
            .field("name", &self.name)
            .field("neighbours", &self.neighbours)
            .finish()
    }
}

fn dive(entry: String, map: &mut HashMap<String, Cave>) -> u32 {
    if entry == "end" {
        1
    } else {
        let mut sum = 0;
        let names = map
            .get(&entry)
            .unwrap()
            .neighbours
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<String>>();
        for cave_name in names {
            let visited_before = map.get(&cave_name).unwrap().visited;
            let do_visit = {
                let cave = map.get_mut(&cave_name).unwrap();
                let is_ok = cave.big || !cave.visited;
                cave.visited = true;
                is_ok
            };
            if do_visit {
                sum += dive(cave_name.clone(), map);
                
            }
            {
                let cave = map.get_mut(&cave_name).unwrap();
                cave.visited = visited_before;
            }

        }
        sum
    }
}

fn main() {
    let file = File::open("day_12/input2").unwrap();
    let reader = BufReader::new(file);
    let mut map = HashMap::<String, Cave>::new();

    // load and parse data, unfortunately lots of cloning
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split('-').collect::<Vec<&str>>();
        let name1 = String::from(line[0]);
        let name2 = String::from(line[1]);
        {
            let cave1 = map.entry(name1.clone()).or_insert(Cave::new(name1.clone()));
            cave1.neighbours.insert(name2.clone());
        }
        {
            let cave2 = map.entry(name2.clone()).or_insert(Cave::new(name2.clone()));
            cave2.neighbours.insert(name1.clone());
        }
    }

    for (cave_name, cave) in map.iter() {
        println!("{:?}", cave);
    }

    map.get_mut("start").unwrap().visited = true;
    let total = dive("start".to_string(), &mut map);
    println!("{}", total);
}
