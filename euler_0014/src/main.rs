use std::collections::HashMap;

fn circuit_length(n: u64, map: HashMap<u64, u64>) 
                  -> (HashMap<u64, u64>, u64) {
    if let Some(&value) = map.get(&n) {
        (map, value)
    } else if n % 2 == 0 {
        let (mut map, sub) = circuit_length(n / 2, map);
        map.insert(n, sub + 1);
        (map, sub + 1)
    } else {
        let (mut map, sub) = circuit_length(3*n + 1, map);
        map.insert(n, sub + 1);
        (map, sub + 1)
    }
}

fn main() {
    let mut circuit_lengths = HashMap::new();
    let mut max_len = 1u64;
    let mut idx_of_max = 0u64;
    circuit_lengths.insert(1u64, 1u64);
    for i in 1u64..1000000u64 {
        let pair = circuit_length(i, circuit_lengths);
        let len = pair.1;
        circuit_lengths = pair.0;
        if len > max_len {
            idx_of_max = i;
            max_len = len;
        }
    }
    println!("{}->{}", idx_of_max, max_len);
}
