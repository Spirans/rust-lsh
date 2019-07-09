mod hash;
use hash::{Hash, HashTableKey, Hyperlanes, QueryResult, Point, HashTableBucket, HashTable};
use core::borrow::{Borrow};
use std::collections::HashMap;

fn euclidean_dist_square(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter().zip(p2).
       fold(0.0, |acc, (i, j)| acc + (j - i).
       powf(2.0))
}

struct CosineLshParam {
    dim: i64,
    l: i64,
    m: i64,
    h: i64,
    hyperplanes: Hyperlanes,
}

impl CosineLshParam {
    fn new(dim: i64, l: i64, m: i64, h: i64, hyperplanes: Hyperlanes)
        -> CosineLshParam {
        CosineLshParam{dim, l, m, hyperplanes, h}
    }

    fn hash(&self, point: &[f64]) -> Vec<HashTableKey> {
        let simhash = Hash::new(self.hyperplanes.borrow(), point);
        let mut hvs :Vec<HashTableKey> = Vec::with_capacity(self.l as usize);
        for i in 0..hvs.capacity() {
            let mut s = Vec::with_capacity(self.m as usize);
            for j in 0..self.m {
                s.insert(j as usize, simhash.sig.0[i*self.m as usize+j as usize].to_owned());
            }
            hvs.insert(i, s);
        };
        hvs
    }
}


pub struct CosineLSH {
    tables: Vec<HashTable>,
    next_id: u64,
    param: CosineLshParam
}

impl CosineLSH {
    pub fn new(dim: i64, l: i64, m: i64) -> Self {
        CosineLSH {
            tables: vec![HashTable::new(); l as usize],
            next_id: 0,
            param: CosineLshParam::new(dim, l, m, m * l, Hyperlanes::new(m * l, dim))
        }
    }

    pub fn insert(&mut self, point: Vec<f64>, extra_data: u64) {
        if let Some(hvs) = self.to_basic_hash_table_keys(self.param.hash(point.as_slice())) {
            for (a, b) in self.tables.iter_mut().enumerate() {
                let j = hvs[a];

                self.next_id += 1;
                b.entry(j).
                    or_insert_with(HashTableBucket::new).
                    push(Point { vector: point.clone(), id: self.next_id, extra_data });
            };
        }
    }

    pub fn query(&self, q: Vec<f64>, max_result: usize) -> Option<Vec<QueryResult>> {
        let mut seen :HashMap<u64,&Point> = HashMap::new();
        if let Some(hvs) = self.to_basic_hash_table_keys(self.param.hash(&q)) {
        self.tables.iter().enumerate().for_each(|(i, table)|
            {
                if let Some(candidates) = table.get(hvs[i].borrow()) {
                    candidates.iter().
                        for_each(|p| { seen.entry(p.id).or_insert(p); });
                }
            }
        );}


        let mut distances :Vec<QueryResult> = Vec::with_capacity(seen.len());
        for (_, value) in seen {
            let distance = euclidean_dist_square(&q, &value.vector);
            distances.push(QueryResult{
                distance,
                vector: &value.vector,
                id: value.id,
                extra_data: value.extra_data});
        }
        if distances.is_empty() { return None }
        distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        if max_result> 0 && distances.len() > max_result as usize {
            Some(distances[0..max_result].to_vec())
        } else {
            Some(distances)
        }
    }

    fn to_basic_hash_table_keys(&self, keys: Vec<HashTableKey>) -> Option<Vec<u64>> {
        let mut basic_keys :Vec<u64> = Vec::with_capacity(self.param.l as usize);
        for (i, key) in keys.iter().enumerate() {
            let mut s = "".to_string();
            for (_, hash_val) in key.iter().enumerate() {
                match hash_val {
                    0 => s.push_str("0"),
                    1 => s.push_str("1"),
                    _ => return None
                }
            }
            basic_keys.insert(i, s.parse().unwrap());
        }
        Some(basic_keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidian_test() {
        let a = vec![34.0,12.0,65.0,29.0];
        let b = vec![2.0,3.0,4.0];
        assert_eq!(euclidean_dist_square(a.as_slice(), b.as_slice()), 4826.0);
    }
}