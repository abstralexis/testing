use std::hash::Hash;

pub struct Set<T> {
    items: Vec<T>,
}

impl<T: Ord + Hash + Clone> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.get_items()
    }
}
impl<T: Ord + Hash + Clone> Set<T> {
    pub fn new() -> Self {
        Set { items: Vec::new() }.remove_duplicates()
    }

    fn remove_duplicates(&self) -> Set<T> {
        let mut items = self.items.clone();
        items.sort();
        items.dedup();
        Set { items: items }
    }

    pub fn get_items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub fn union(&self, set: &Set<T>) -> Set<T> {
        let mut items = self.items.clone();
        items.append(&mut set.get_items());
        Set { items: items }.remove_duplicates()
    }

    pub fn intersect(&self, set: &Set<T>) -> Set<T> {
        let items: Vec<T> = self
            .items
            .clone()
            .into_iter()
            .filter(|a| set.get_items().contains(a))
            .collect();
        Set { items: items }.remove_duplicates()
    }

    pub fn subset(&self, set: &Set<T>) -> bool {
        for item in self.get_items().into_iter() {
            if !set.get_items().contains(&item) {
                return false;
            }
        }
        true
    }

    pub fn proper_subset(&self, set: &Set<T>) -> bool {
        self.subset(&set) && self != set
    }
}

fn main() {
    println!("Hello, world!");
}
