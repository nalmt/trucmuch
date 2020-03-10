use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

const BLOCK_SIZE: usize = 8;

pub struct Set {
    data: Vec<u8>,
}

pub trait BitSet<T> {
    fn empty() -> Self;

    /// Set the item to 1.
    fn insert(&mut self, item: T);

    fn calculate_hash(t: &T) -> u64;

    /// If the item is set, clear the item to 0.
    /// Do nothing otherwise.
    fn clear(&mut self, item: T);

    /// Toggle the item.
    fn contains(&self, item: T) -> bool;

    /// Return a new BitSet with one item set.
    fn unit(item: T) -> Self;

    /// Return a new BitSet with two items set.
    fn two(item1: T, item2: T) -> Self;

    fn card(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn intersect(&mut self, other: &Self);
    fn union(&mut self, other: &Self);
    fn remove(&mut self, other: &Self);
    //  fn intersects(&self, other: &Self) -> bool;
    //  fn is_subset_of(&self, other: &Self) -> bool;
    //  fn to_vec(&self) -> Vec<T>;
     fn from_vec(vec: Vec<T>) -> Self;
    fn equals(&self, other: &Self) -> bool;
    // fn retain<F>(&mut self, f: F);

    //  fn map<F, U>(&self, f: F) -> Set<U>;
    //  fn filter_map<F, U>(&self, f: F) -> Set<U>;
}

impl<T: Eq + Ord + Copy + Hash + Debug> BitSet<T> for Set {
    fn calculate_hash(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    fn empty() -> Self {
        Self {
            data: Vec::<u8>::new(),
        }
    }

    fn insert(&mut self, item: T) {
        let mut key = Self::calculate_hash(&item) as usize;

        key = key % 500;
        println!("hash: {}", key);

        let block_number = key / BLOCK_SIZE;
        let block_index = key % BLOCK_SIZE;

        println!("block_number: {}", block_number);
        println!("block_index: {}", block_index);

        let mut data_size = self.data.len();

        while data_size < (block_number + 1) {
            self.data.push(0);
            data_size += 1;
        }

        self.data[block_number] |= 1 << block_index;
    }

    fn clear(&mut self, item: T) {
        let mut key = Self::calculate_hash(&item) as usize;

        key = key % 500;
        println!("hash: {}", key);

        let block_number = key / BLOCK_SIZE;
        let block_index = key % BLOCK_SIZE;

        println!("block_number: {}", block_number);
        println!("block_index: {}", block_index);

        let data_size = self.data.len();

        if data_size < block_number + 1 {
            return;
        }

        self.data[block_number] &= !(1 << block_index);
    }

    fn contains(&self, item: T) -> bool {


        let mut key = Self::calculate_hash(&item) as usize;

        key = key % 500;
        println!("hash: {}", key);

        let block_number = key / BLOCK_SIZE;
        let block_index = key % BLOCK_SIZE;

        (1 & (self.data[block_number] >> block_index)) != 0
    }

    fn unit(item: T) -> Self {
        let mut s: Set = BitSet::<T>::empty();
        s.insert(item);
        s
    }

    fn two(item1: T, item2: T) -> Self {
        let mut s: Set = BitSet::<T>::empty();
        s.insert(item1);
        s.insert(item2);
        s
    }

    fn card(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn intersect(&mut self, other: &Self) {
        let mut res: Set = BitSet::<T>::empty();

        for item in self.data.iter() {
            if other.data.contains(item) {
                res.insert(*item);
            }
        }
        self.data = res.data;
    }

    fn union(&mut self, other: &Self) {
        for item in other.data.iter() {
            self.insert(*item);
        }
    }

    fn remove(&mut self, other: &Self) {
        for item in other.data.iter() {
            self.clear(item);
        }
    }

    //   fn to_vec(&self) -> Vec<T> {
    //       let mut res = Vec::<T>::new();
    //       for item in self.data.iter() {
    //         res.push(*item)
    //       }
    //       res.sort_unstable();
    //       res
    //     }

      fn from_vec(vec: Vec<T>) -> Self {
          let mut res: Set = BitSet::<T>::empty();
          for x in vec {
            res.insert(x);
          }
          res
        }

      fn equals(&self, other: &Self) -> bool {
          self.data == other.data
        }

    //   fn retain<F>(&mut self, f: F)
    //     where
    //       F: FnMut(&T) -> bool,
    //     {
    //       self.data.retain(f)
    //     }
}

fn main() {
    let mut truc: Set = BitSet::<u8>::empty();

    truc.insert(8);
    assert_eq!(truc.data[0], 0);
    assert_eq!(truc.data[1], 0);
    assert_eq!(truc.data[22], 0);
    assert_eq!(truc.data[23], 8);

    let _truc1: Set = BitSet::<u8>::unit(9);
    let _truc2: Set = BitSet::<u8>::two(10, 15);
}
