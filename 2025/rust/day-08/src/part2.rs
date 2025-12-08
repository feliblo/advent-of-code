use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

pub struct UnionFind<T: Debug + Eq + Hash> {
    payloads: HashMap<T, usize>, // Maps values to their indices in the parent_links array.
    parent_links: Vec<usize>, // Holds the parent pointers; root elements are their own parents.
    sizes: Vec<usize>, // Holds the sizes of the sets.
    count: usize,      // Number of disjoint sets.
}

impl<T: Debug + Eq + Hash> UnionFind<T> {
    /// Creates an empty Union-Find structure with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parent_links: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
            payloads: HashMap::with_capacity(capacity),
            count: 0,
        }
    }

    /// Inserts a new item (disjoint set) into the data structure.
    pub fn insert(&mut self, item: T) {
        let key = self.payloads.len();
        self.parent_links.push(key);
        self.sizes.push(1);
        self.payloads.insert(item, key);
        self.count += 1;
    }

    /// Returns the root index of the set containing the given value, or `None` if it doesn't exist.
    pub fn find(&mut self, value: &T) -> Option<usize> {
        self.payloads
            .get(value)
            .copied()
            .map(|key| self.find_by_key(key))
    }

    /// Unites the sets containing the two given values. Returns:
    /// - `None` if either value hasn't been inserted,
    /// - `Some(true)` if two disjoint sets have been merged,
    /// - `Some(false)` if both elements were already in the same set.
    pub fn union(
        &mut self,
        first_item: &T,
        sec_item: &T,
    ) -> Option<bool> {
        let (first_root, sec_root) = (
            self.find(first_item),
            self.find(sec_item),
        );
        match (first_root, sec_root) {
            (Some(first_root), Some(sec_root)) => Some(
                self.union_by_key(first_root, sec_root),
            ),
            _ => None,
        }
    }

    /// Finds the root of the set containing the element with the given index.
    fn find_by_key(&mut self, key: usize) -> usize {
        if self.parent_links[key] != key {
            self.parent_links[key] =
                self.find_by_key(self.parent_links[key]);
        }
        self.parent_links[key]
    }

    /// Unites the sets containing the two elements identified by their indices.
    fn union_by_key(
        &mut self,
        first_key: usize,
        sec_key: usize,
    ) -> bool {
        let (first_root, sec_root) = (
            self.find_by_key(first_key),
            self.find_by_key(sec_key),
        );

        if first_root == sec_root {
            return false;
        }

        match self.sizes[first_root]
            .cmp(&self.sizes[sec_root])
        {
            Ordering::Less => {
                self.parent_links[first_root] = sec_root;
                self.sizes[sec_root] +=
                    self.sizes[first_root];
            }
            _ => {
                self.parent_links[sec_root] = first_root;
                self.sizes[first_root] +=
                    self.sizes[sec_root];
            }
        }

        self.count -= 1;
        true
    }

    /// Checks if two items belong to the same set.
    pub fn is_same_set(
        &mut self,
        first_item: &T,
        sec_item: &T,
    ) -> bool {
        matches!((self.find(first_item), self.find(sec_item)), (Some(first_root), Some(sec_root)) if first_root == sec_root)
    }

    /// Returns the number of disjoint sets.
    pub fn count(&self) -> usize {
        self.count
    }
}

impl<T: Debug + Eq + Hash> Default for UnionFind<T> {
    fn default() -> Self {
        Self {
            parent_links: Vec::default(),
            sizes: Vec::default(),
            payloads: HashMap::default(),
            count: 0,
        }
    }
}

impl<T: Debug + Eq + Hash> FromIterator<T>
    for UnionFind<T>
{
    /// Creates a new UnionFind data structure from an iterable of disjoint elements.
    fn from_iter<I: IntoIterator<Item = T>>(
        iter: I,
    ) -> Self {
        let mut uf = UnionFind::default();
        for item in iter {
            uf.insert(item);
        }
        uf
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

struct JunctionBox {
    x: i32,
    y: i32,
    z: i32,
}

impl JunctionBox {
    fn calculate_distances(
        self,
        other: &JunctionBox,
    ) -> f32 {
        let dx = (other.x - self.x) as f32;
        let dy = (other.y - self.y) as f32;
        let dz = (other.z - self.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let objects: Vec<JunctionBox> = input
        .lines()
        .map(|l| {
            let mut split = l.split(",");
            JunctionBox {
                x: FromStr::from_str(split.next().unwrap())
                    .unwrap(),
                y: FromStr::from_str(split.next().unwrap())
                    .unwrap(),
                z: FromStr::from_str(split.next().unwrap())
                    .unwrap(),
            }
        })
        .collect();

    // Generate all unique pairs with distances
    let mut all_pairs: Vec<(usize, usize, f32)> =
        Vec::new();
    for (i, box_i) in objects.iter().enumerate() {
        for (j, box_j) in objects.iter().enumerate() {
            if i < j {
                let d = box_i.calculate_distances(box_j);
                all_pairs.push((i, j, d));
            }
        }
    }

    // Sort all pairs by distance ascending
    all_pairs
        .sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Initialize union-find and insert all junction boxes
    let mut uf: UnionFind<JunctionBox> =
        UnionFind::with_capacity(objects.len());
    for jb in objects.iter().cloned() {
        uf.insert(jb);
    }

    // Connections made
    let mut last_pair = None;
    for &(i, j, _) in &all_pairs {
        let jb1 = &objects[i];
        let jb2 = &objects[j];
        if uf.union(jb1, jb2) == Some(true) {
            last_pair = Some((jb1, jb2));
            if uf.count() == 1 {
                break;
            }
        }
    }

    if let Some((jb1, jb2)) = last_pair {
        let result = (jb1.x as usize) * (jb2.x as usize);
        return Ok(result.to_string());
    }
    Ok("I failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!("25272", process(input)?);
        Ok(())
    }
}
