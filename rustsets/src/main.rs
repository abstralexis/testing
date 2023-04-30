use std::hash::Hash;

pub trait SetType<T: Ord + Hash + Clone> {
    fn eq(&self, other: &Self) -> bool;
    fn new() -> Self;
    fn push(&self, items: &Vec<T>) -> Set<T>;
    fn remove_duplicates(&self) -> Set<T>;
    fn get_items(&self) -> Vec<T>;
    fn union(&self, set: &Set<T>) -> Set<T>;
    fn intersect(&self, set: &Set<T>) -> Set<T>;
    fn subset(&self, set: &Set<T>) -> bool;
    fn proper_subset(&self, set: &Set<T>) -> bool;
}

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

    pub fn push(&self, items: &Vec<T>) -> Set<T> {
        let mut old: Vec<T> = self.get_items();
        let mut new: Vec<T> = items.clone();
        old.append(&mut new);

        Set {
            items: old
        }.remove_duplicates()
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

#[cfg(test)]
mod test {
    use crate::Set;

    #[test]
    fn union_test() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        let set_b: Set<i32> = Set::new().push(&vec![3, 4, 5]);
        let expected_set: Set<i32> = Set::new().push(&vec![1, 2, 3, 4, 5]);
        assert!(set_a.union(&set_b) == expected_set);
    }

    #[test]
    fn intersection_test() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        let set_b: Set<i32> = Set::new().push(&vec![2, 3, 4]);
        let expected_set: Set<i32> = Set::new().push(&vec![2, 3]);
        assert!(set_a.intersect(&set_b) == expected_set);
    }

    #[test]
    fn improper_subset_true_proper() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        let set_b: Set<i32> = Set::new().push(&vec![1, 2, 3, 4, 5]);
        assert!(set_a.subset(&set_b) == true);
    }

    #[test]
    fn improper_subset_true_improper() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        let set_b: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        assert!(set_a.subset(&set_b) == true);
    }

    #[test]
    fn improper_subset_false() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2]);
        let set_b: Set<i32> = Set::new().push(&vec![3, 4, 5]);
        assert!(set_a.subset(&set_b) == false)
    }

    #[test]
    fn proper_subset_true() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2]);
        let set_b: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        assert!(set_a.proper_subset(&set_b) == true);
    }

    #[test]
    fn proper_subset_false() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2]);
        let set_b: Set<i32> = Set::new().push(&vec![4, 5, 6]);
        assert!(set_a.proper_subset(&set_b) == false);
    }

    #[test]
    fn proper_subset_improper() {
        let set_a: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        let set_b: Set<i32> = Set::new().push(&vec![1, 2, 3]);
        assert!(set_a.proper_subset(&set_b) == false);
    }
}

fn main() {
    println!("Hello, world!");
}