use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn solve_mapping(ins: &Vec<HashSet<char>>) -> (HashMap<char, char>, HashMap<char, char>) {
    let mut jumbled_to_real: HashMap<char, char> = HashMap::new();
    let mut real_to_jumbled: HashMap<char, char> = HashMap::new();
    let num_one = ins.iter().filter(|s| s.len() == 2).next().expect("must be a zero");
    let num_seven = ins.iter().filter(|s| s.len() == 3).next().expect("must be a one");
    //Segment a is in 7 but not 1t
    let maps_to_a = *num_seven.difference(&num_one).next().expect("diff");
    jumbled_to_real.insert(maps_to_a, 'a');
    real_to_jumbled.insert('a', maps_to_a);
    let cf_counts: HashMap<usize, char> = num_one.iter().map(|seg| {
        (ins.iter().filter(|s| s.contains(seg)).count(), *seg)
    }).collect();
    let maps_to_c = *cf_counts.get(&8).unwrap();
    jumbled_to_real.insert(maps_to_c, 'c');
    real_to_jumbled.insert('c', maps_to_c);
    let maps_to_f = *cf_counts.get(&9).unwrap();
    jumbled_to_real.insert(maps_to_f, 'f');
    real_to_jumbled.insert('f', maps_to_f);

    let num_four = ins.iter().filter(|s| s.len() == 4).next().expect("must be a four");
    let bd_counts: HashMap<usize, char> = num_four.difference(&num_one).map(|seg| {
        (ins.iter().filter(|s| s.contains(seg)).count(), *seg)
    }).collect();
    let maps_to_b = *bd_counts.get(&6).unwrap();
    jumbled_to_real.insert(maps_to_b, 'b');
    real_to_jumbled.insert('b', maps_to_b);
    let maps_to_d = *bd_counts.get(&7).unwrap();
    jumbled_to_real.insert(maps_to_d, 'd');
    real_to_jumbled.insert('d', maps_to_d);

    //Inserect the 6 segments (0 6 9) gives us abfg. We know adf to get g
    let sets_069: Vec<&HashSet<char>> = ins.iter().filter(|s| s.len() == 6).collect();
    let mut set_abfg: HashSet<char> = sets_069[0].intersection(sets_069[1]).filter(|i| sets_069[2].contains(i))
        .map(|i| *i).collect();
    set_abfg.remove(real_to_jumbled.get(&'a').unwrap());
    set_abfg.remove(real_to_jumbled.get(&'b').unwrap());
    set_abfg.remove(real_to_jumbled.get(&'f').unwrap());
    let maps_to_g = *set_abfg.iter().next().unwrap();
    jumbled_to_real.insert(maps_to_g, 'g');
    real_to_jumbled.insert('g', maps_to_g);

    // Now e is the only unknown segment in 8, which is the only input with all 7 segments
    let num_eight = ins.iter().filter(|s| s.len() == 7).next().expect("must be a eight");
    let known: HashSet<char> = jumbled_to_real.keys().map(|k| *k).collect();
    let maps_to_e = *num_eight.difference(&known).next().expect("diff");
    jumbled_to_real.insert(maps_to_e, 'e');
    real_to_jumbled.insert('e', maps_to_e);
    
    (jumbled_to_real, real_to_jumbled)
}

fn parse_part(part: &str) -> Vec<HashSet<char>> {
    part.split(" ").map(|sec| {
        sec.chars().collect()
    }).collect()
}

fn main() {
    let mappings = [
        HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']),
        HashSet::from(['c', 'f']),
        HashSet::from(['a', 'c', 'd', 'e', 'g']),
        HashSet::from(['a', 'c', 'd', 'f', 'g']),
        HashSet::from(['b', 'c', 'd', 'f']),
        HashSet::from(['a', 'b', 'd', 'f', 'g']),
        HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']),
        HashSet::from(['a', 'c', 'f']),
        HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']), 
        HashSet::from(['a', 'b', 'c', 'd', 'f', 'g'])
    ];
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let problems: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> = input.lines().map(|line| {
        let halfs: Vec<&str> = line.split(" | ").collect();
        (parse_part(halfs[0]), parse_part(halfs[1]))
    }).collect();

    let mut counts = 0;
    for problem in problems {
        let (ins, outs) = problem;
        let (j_to_r, _) = solve_mapping(&ins);
        let mut current = 0;
        for i in 0..4 {
            current *= 10;
            let mapped: HashSet<char> = outs[i].iter().map(|c| *j_to_r.get(c).unwrap()).collect();
            for i in 0..=9 {
                if mappings[i] == mapped {
                    current += i;
                }
            }
        }
        counts += current;
    }
    println!("{}", counts);
}
