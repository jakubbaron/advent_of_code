pub struct Reindeer {
    speed: u32,
    run_time: u32,
    rest_time: u32,
}

impl Reindeer {
    pub fn new(speed: u32, run_time: u32, rest_time: u32) -> Reindeer {
        Reindeer {
            speed,
            run_time,
            rest_time,
        }
    }

    pub fn run_for(&self, time: u32) -> u32 {
        let mut run_timer = 0;
        let mut rest_timer = 0;
        let mut total_distance = 0;
        let mut t = 0;
        while t < time {
            t += 1;
            if run_timer < self.run_time {
                run_timer += 1;
                total_distance += self.speed;
            } else if rest_timer < self.rest_time {
                rest_timer += 1;
            }

            if rest_timer == self.rest_time {
                run_timer = 0;
                rest_timer = 0;
            }
            // println!("{}: dist: {} rest: {}, run: {}", t, total_distance, rest_timer, run_timer);
        }
        total_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comet() {
        let r = Reindeer {
            speed: 14,
            run_time: 10,
            rest_time: 127,
        };
        assert_eq!(1120, r.run_for(1000));
    }

    #[test]
    fn test_dancer() {
        let r = Reindeer {
            speed: 16,
            run_time: 11,
            rest_time: 162,
        };
        assert_eq!(1056, r.run_for(1000));
    }
}
