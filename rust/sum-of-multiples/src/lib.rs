pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let multiples_vecs = factors.iter().map(|fac| multiples_n(limit, *fac)).collect();
    let multiples_vecs_set = to_set(multiples_vecs);
    multiples_vecs_set.iter().sum()
}

fn multiples_n(limit: u32, n: u32) -> Vec<u32> {
    let multiples = (1..(limit as f32 / n as f32) as u32 + 1)
        .collect::<Vec<u32>>()
        .iter()
        .map(|i| {
            if i * n == limit {
                return 0;
            }
            i * n
        }).collect::<Vec<u32>>();
    multiples
}

fn to_set(vectors: Vec<Vec<u32>>) -> Vec<u32> {
    let mut flat = Vec::new();
    for vec in vectors {
        flat.extend(vec);
    }
    flat.sort();
    flat.dedup_by(|a, b| *a == *b);
    flat
}
