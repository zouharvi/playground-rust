pub fn is_mutable(a: &str, b: &str) -> bool {
    let mut differed = false;
    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            if differed {
                return false;
            }
            differed = true;
        }
    }
    true
}

pub fn bank_graph(bank: &Vec<String>) -> Vec<Vec<usize>> {
    let mut out: Vec<_> = bank.iter().map(|_x| Vec::<usize>::new()).collect();

    for (g1_i, g1) in bank.iter().enumerate() {
        for (g2_i, g2) in bank[(g1_i + 1)..].iter().enumerate() {
            let g2_i = g1_i + g2_i + 1;
            if is_mutable(g1, g2) {
                out[g1_i].push(g2_i);
                out[g2_i].push(g1_i);
            }
        }
    }

    out
}

pub fn min_mutation(start: String, end: String, mut bank: Vec<String>) -> i32 {
    // the end is not valid
    if bank.iter().all(|x| *x != end) {
        return -1;
    }

    // make sure that start and end are in the bank
    if bank.iter().all(|s| s != &start) {
        bank.push(start.clone());
    }
    if bank.iter().all(|s| s != &end) {
        bank.push(end.clone());
    }

    let graph = bank_graph(&bank);

    let end_i = bank.iter().position(|r| *r == end).unwrap();
    let start_i = bank.iter().position(|r| *r == start).unwrap();
    let mut distance = 0;

    // now try to see if we can get from 0 to bank.len()-1
    let mut queue = [start_i].to_vec();
    let mut next_queue = Vec::<usize>::new();
    let mut visited: Vec<bool> = bank.iter().map(|_| false).collect();

    loop {
        if let Some(cur_i) = queue.pop() {
            // the queue is not empty yet
            visited[cur_i] = true;
            if cur_i == end_i {
                return distance;
            }
            for neighbour_i in graph[cur_i].iter() {
                if !visited[*neighbour_i] {
                    next_queue.push(*neighbour_i);
                }
            }
        } else if !next_queue.is_empty() {
            // go to the next wave
            distance += 1;
            queue = next_queue;
            next_queue = vec![];
        } else {
            // exhausted all options
            return -1;
        }
    }
}

pub fn main() {
    let bank = ["AACCGGTA"].iter().map(|x| x.to_string()).collect();
    println!(
        "{:?}",
        min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), bank)
    );

    let bank = ["AACCGGTA", "AACCGCTA", "AAACGGTA"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!(
        "{:?}",
        min_mutation("AACCGGTT".to_string(), "AAACGGTA".to_string(), bank)
    );

    let bank = ["AAAACCCC", "AAACCCCC", "AACCCCCC"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!(
        "{:?}",
        min_mutation("AAAAACCC".to_string(), "AACCCCCC".to_string(), bank)
    );

    let bank = [].to_vec();
    println!(
        "{:?}",
        min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), bank)
    );
}
