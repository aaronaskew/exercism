#[derive(Debug, Eq)]
pub struct CustomSet<T: PartialEq + Clone> {
    data: Vec<T>,
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter().all(|e| other.data.contains(e))
            && other.data.iter().all(|e| self.data.contains(e))
    }
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        Self {
            data: Vec::from(_input),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.iter().all(|element| !other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|element| other.contains(element))
                .cloned()
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|element| !other.contains(element))
                .cloned()
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::new(&[]);

        for element in self.data.clone() {
            union.add(element);
        }

        for element in other.data.clone() {
            union.add(element);
        }

        union
    }
}
