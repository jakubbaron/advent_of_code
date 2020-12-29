pub struct Spoon {
    _name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: usize,
}

impl Spoon {
    pub fn new(
        name: &str,
        capacity: i64,
        durability: i64,
        flavor: i64,
        texture: i64,
        calories: usize,
    ) -> Spoon {
        Spoon {
            _name: name.to_string(),
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
    pub fn as_vec(&self) -> Vec<i64> {
        vec![self.capacity, self.durability, self.flavor, self.texture]
    }
    pub fn get_calories(&self) -> usize {
        self.calories
    }
}

pub fn get_score(vec: &Vec<i64>) -> i64 {
    let mut score = 1;
    for el in vec {
        if *el < 0 {
            return 0;
        }
        score *= *el;
    }
    score
}

pub fn add_spoon(vec: &Vec<i64>, spoon: &Spoon) -> Vec<i64> {
    let mut copy = vec.to_vec();
    let spoon_vec = spoon.as_vec();
    for i in 0..copy.len() {
        copy[i] += spoon_vec[i];
    }
    copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_score() {
        let v = vec![68, 80, 152, 76];
        assert_eq!(get_score(&v), 62842880);
    }

    #[test]
    fn test_get_score_negative() {
        let v = vec![68, 80, -152, 76];
        assert_eq!(get_score(&v), 0);
    }

    #[test]
    fn test_as_vec() {
        let s = Spoon::new("abc", 1, 2, 3, 4, 5);
        assert_eq!(s.as_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_score_of_spoon_1() {
        let s = Spoon::new("abc", -1, -2, 6, 3, 8);
        assert_eq!(get_score(&s.as_vec()), 0);
    }
    #[test]
    fn test_score_of_spoon_2() {
        let s = Spoon::new("abc", 2, 3, -2, -1, 3);
        assert_eq!(get_score(&s.as_vec()), 0);
    }

    #[test]
    fn test_add_spoon() {
        let mut v = vec![68, 80, 152, 76];
        let s = Spoon::new("abc", 2, 3, -2, -1, 3);
        v = add_spoon(&v, &s);
        assert_eq!(vec![70, 83, 150, 75], v);
    }
}
