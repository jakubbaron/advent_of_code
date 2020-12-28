#[derive(Clone)]
pub struct Reindeer {
    speed: u32,
    run_time: u32,
    rest_time: u32,
    run_timer: u32,
    rest_timer: u32,
    total_distance: u32,
    points: u32,
}

impl Reindeer {
    pub fn new(speed: u32, run_time: u32, rest_time: u32) -> Reindeer {
        Reindeer {
            speed,
            run_time,
            rest_time,
            rest_timer: 0,
            run_timer: 0,
            total_distance: 0,
            points: 0,
        }
    }

    pub fn run_for(&mut self, time: u32) -> u32 {
        let mut t = 0;
        while t < time {
            t += 1;
            self.run();
            // println!("{}: dist: {} rest: {}, run: {}", t, total_distance, rest_timer, run_timer);
        }
        self.total_distance
    }
    pub fn run(&mut self) -> u32 {
        if self.run_timer < self.run_time {
            self.run_timer += 1;
            self.total_distance += self.speed;
        } else if self.rest_timer < self.rest_time {
            self.rest_timer += 1;
        }

        if self.rest_timer == self.rest_time {
            self.run_timer = 0;
            self.rest_timer = 0;
        }
        self.total_distance
    }

    pub fn add_point(&mut self) {
        self.points += 1;
    }

    pub fn get_points(&self) -> u32 {
        self.points
    }

    pub fn get_distance(&self) -> u32 {
        self.total_distance
    }
}
pub fn race_reindeers(reindeers: &mut Vec<Reindeer>, time: u32) {
    let mut distances: Vec<u32> = vec![0; reindeers.len()];
    for _ in 0..time {
        for i in 0..reindeers.len() {
            distances[i] = reindeers[i].run();
        }
        let max_distance = *distances.iter().max().unwrap();
        for i in 0..reindeers.len() {
            if reindeers[i].get_distance() == max_distance {
                reindeers[i].add_point();
            }
        }
    }
}

pub fn get_max_points(reindeers: &Vec<Reindeer>) -> u32 {
    reindeers.iter().map(|x| x.get_points()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comet() {
        let mut r = Reindeer::new(14, 10, 127);
        assert_eq!(1120, r.run_for(1000));
    }

    #[test]
    fn test_dancer() {
        let mut r = Reindeer::new(16, 11, 162);
        assert_eq!(1056, r.run_for(1000));
    }

    #[test]
    fn test_race() {
        let mut reindeers = vec![Reindeer::new(14, 10, 127), Reindeer::new(16, 11, 162)];
        race_reindeers(&mut reindeers, 1_000);
        assert_eq!(reindeers[0].get_points(), 312);
        assert_eq!(reindeers[1].get_points(), 689);

        assert_eq!(get_max_points(&reindeers), 689);
    }

    #[test]
    fn test_reindeer_clone() {
        let mut r1 = Reindeer::new(1, 1, 1);
        let r2 = r1.clone();
        r1.run();
        assert_eq!(r1.get_distance(), 1);
        assert_ne!(r1.get_distance(), r2.get_distance());
    }
}
