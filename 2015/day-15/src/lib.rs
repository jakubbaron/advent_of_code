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

fn get_score(vec: &Vec<i64>) -> i64 {
    let mut score = 1;
    for el in vec {
        if *el < 0 {
            return 0;
        }
        score *= *el;
    }
    score
}

fn add_spoon(vec: &Vec<i64>, spoon: &Spoon) -> Vec<i64> {
    let mut copy = vec.to_vec();
    let spoon_vec = spoon.as_vec();
    for i in 0..copy.len() {
        copy[i] += spoon_vec[i];
    }
    copy
}

pub fn optimize_for_score(spoons_vec: &Vec<Spoon>, iterations: usize) -> i64 {
    assert!(spoons_vec.len() > 0);
    assert!(spoons_vec.len() < iterations);

    let mut scores: Vec<i64> = vec![0; spoons_vec[0].as_vec().len()];
    let mut used_spoons = 0;
    for spoon in spoons_vec.iter() {
        scores = add_spoon(&scores, &spoon);
        used_spoons += 1;
    }

    let mut res_1 = 0;
    for _ in used_spoons..iterations {
        let (score, new_scores) = spoons_vec
            .iter()
            .map(|spoon| add_spoon(&scores, &spoon))
            .map(|new_scores| (get_score(&new_scores), new_scores))
            .max()
            .unwrap();
        scores = new_scores;
        res_1 = score;
    }
    res_1
}

pub fn optimize_for_score_and_kcals(
    spoons_vec: &Vec<Spoon>,
    required_kcals: usize,
    iterations: usize,
) -> i64 {
    assert!(spoons_vec.len() > 0);
    assert!(spoons_vec.len() < iterations);
    let mut scores: Vec<i64> = vec![0; spoons_vec[0].as_vec().len()];
    let mut used_spoons = 0;
    let mut used_kcals = 0;
    for spoon in spoons_vec.iter() {
        scores = add_spoon(&scores, &spoon);
        used_spoons += 1;
        used_kcals += spoon.get_calories();
    }
    assert!(used_kcals <= required_kcals);

    let mut calories_scores = vec![vec![(0, vec![0]); required_kcals + 1]; iterations + 1];
    calories_scores[used_spoons][used_kcals] = (get_score(&scores), scores);
    for spoons in used_spoons + 1..iterations + 1 {
        for kcals in used_kcals + 1..required_kcals + 1 {
            let mut max_score = 0;
            let mut max_vec: Vec<i64> = Vec::new();
            for spoon in spoons_vec.iter() {
                if spoon.get_calories() >= kcals {
                    continue;
                } else {
                    let (_, scores) = &calories_scores[spoons - 1][kcals - spoon.get_calories()];
                    let tmp = add_spoon(&scores, &spoon);
                    let tmp_result = get_score(&tmp);
                    if tmp_result > max_score {
                        max_score = tmp_result;
                        max_vec = tmp;
                    }
                }
            }
            calories_scores[spoons][kcals] = (max_score, max_vec);
        }
    }
    // println!("[{}][{}]: {:?}", 100, 500, calories_scores[100][500]);
    calories_scores[iterations][required_kcals].0
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
