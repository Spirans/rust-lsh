use std::collections::HashMap;
extern crate rand;

use rand::Rng;

pub struct Signature(pub Vec<u8>);
pub type HashTableKey = Vec<u8>;

#[derive(Debug,Clone)]
pub struct Point<T: Sized>{
    pub vector: Vec<f64>,
    pub extra_data: T,
    pub id: u64,
}

#[derive(Debug, Clone)]
pub struct QueryResult<'a, T> {
    pub distance: f64,
    pub vector: &'a [f64],
    pub extra_data: T,
    pub id: u64,
}

pub type HashTableBucket<T> = Vec<Point<T>>;

pub type HashTable<T> = HashMap<u64, HashTableBucket<T>>;

pub struct Hyperlanes(Vec<Vec<f64>>);

pub struct Hash {
    pub sig: Signature,
}

impl Hash {
    pub fn new(hs: &Hyperlanes, e: &[f64]) -> Self {
        Hash {sig: Signature::new(hs, e)}
    }
}

impl Signature {
    fn new(hs: &Hyperlanes, e: &[f64]) -> Self {
        let mut sigarr: Vec<u8> = vec![0; hs.0.capacity()];
        hs.0.iter().enumerate().for_each(|(hix, h)| {
            match e.iter().enumerate().
                fold(0.0, |sum, x| sum + h[x.0] * *x.1) {
                d if d >= 0.0 => sigarr.insert(hix, 1),
                _ => sigarr.insert(hix, 0),
            }
        });
        Signature(sigarr)
    }
}

impl Hyperlanes {
    pub fn new(d: i64, s: i64) -> Self {
        let mut rng = rand::thread_rng();
        let mut hs = vec![vec![0.0; s as usize]; d as usize];
        (0..d).for_each(|i| {
            let mut v = vec![0.0; s as usize];
            (0..s).for_each(|i| v.insert(i as usize, rng.gen()));
            hs.insert(i as usize, v);
        });
        Hyperlanes(hs)
    }
}