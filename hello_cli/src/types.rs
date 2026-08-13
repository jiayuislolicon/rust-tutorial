use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::size_of;

fn main() {
    let mut h: HashMap<&str, i32> = HashMap::new();
    let mut b: BTreeMap<&str, i32> = BTreeMap::new();
    let mut seen: HashSet<&str> = HashSet::new();

    for (k, v) in [("pear", 1), ("apple", 3), ("mango", 2), ("kiwi", 5)] {
        h.insert(k, v);
        b.insert(k, v);
    }

    println!("{:?}", h.keys().collect::<Vec<_>>()); // random order
    println!("{:?}", b.keys().collect::<Vec<_>>()); // apple, kiwi, mango, pear
    println!("{}", seen.insert("apple")); // true  ← 第一次，是新的
    println!("{}", seen.insert("apple")); // false ← 已經有了
    println!("{}", seen.len()); // 1

    println!("{}", size_of::<[i32; 3]>()); // 12
    println!("{}", size_of::<Vec<i32>>()); // 24
    println!("{}", size_of::<Vec<String>>()); // 24
    println!("{}", size_of::<String>()); // 24
    println!("{}", size_of::<&str>()); // 16
    println!("{}", size_of::<&i32>()); //  8
}
