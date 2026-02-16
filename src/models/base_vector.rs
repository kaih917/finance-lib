struct Element {
    value: f64,
    index: String,
}


use std::collections::HashSet;

struct BaseVector {
    col_names: HashSet<String>,
    data: HashMap<String, f64>,
}

impl BaseVector {
    fn new(mut colums : HashMap) -> Self {
        BaseVector {
            col_names: HashSet::new(),
            data: HashSet::new(),
        }
    }

    fn get_data() -> &HashSet<Element> {
        &self.data
    }
}

impl Add for BaseVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.get_data();

        for elem in result.iter() {
            result.add(elem.value, elem.index.clone());
        }

        result
    }
}



