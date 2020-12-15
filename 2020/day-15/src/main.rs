use std::collections::HashMap;

struct InputResult {
    input: Vec<usize>,
    result_1: usize,
    result_2: usize,
}

struct LastSpoken {
    zero:usize,
    one:usize,
}

impl LastSpoken {
    fn new() -> LastSpoken {
        LastSpoken{zero:0, one:0}
    }
    fn is_new(&self) -> bool {
        self.zero == 0
    }
    fn push(&mut self, turn: usize) {
        self.zero = self.one;
        self.one = turn;
    }
    fn diff(&self) -> usize {
        self.one - self.zero
    }
}

fn main() {
    let inputs = vec![
        InputResult{input: vec![0,3,6], result_1: 436, result_2:175594},
        InputResult{input: vec![1,3,2], result_1: 1, result_2:2578},
        InputResult{input: vec![2,1,3], result_1: 10, result_2:3544142},
        InputResult{input: vec![1,2,3], result_1: 27, result_2:261214},
        InputResult{input: vec![2,3,1], result_1: 78, result_2:6895259},
        InputResult{input: vec![3,2,1], result_1: 438, result_2:18},
        InputResult{input: vec![3,1,2], result_1: 1836, result_2:362},
        InputResult{input: vec![14,8,16,0,1,17], result_1: 240, result_2:505},
    ];
    for ir in inputs.iter() {
        let InputResult {input, result_1, result_2 } = ir;
        println!("{:?}", input);
        let mut tracker: HashMap<usize, LastSpoken> = HashMap::new();
        for (i, val) in input.iter().enumerate() {
            let mut l = LastSpoken::new();
            l.push(i+1);
            tracker.insert(*val, l);
        }
        let mut last_spoken = *input.last().unwrap();
        for turn in (input.len() + 1)..30000001 {
            tracker.entry(last_spoken).or_insert_with(LastSpoken::new);
            let last_tracker = tracker.get(&last_spoken).unwrap();

            if last_tracker.is_new() {
                last_spoken = 0;
            } else {
                last_spoken = last_tracker.diff();
            }
            tracker.entry(last_spoken).or_insert_with(LastSpoken::new).push(turn);
            if turn == 2020 {
                assert_eq!(last_spoken, *result_1);
            }
            if turn == 30000000 {
                assert_eq!(last_spoken, *result_2);
            }
        }
    }
}
