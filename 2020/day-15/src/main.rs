struct InputResult {
    input: Vec<usize>,
    result_1: usize,
    result_2: usize,
}

fn main() {
    let inputs = vec![
        InputResult {
            input: vec![0, 3, 6],
            result_1: 436,
            result_2: 175594,
        },
        InputResult {
            input: vec![1, 3, 2],
            result_1: 1,
            result_2: 2578,
        },
        InputResult {
            input: vec![2, 1, 3],
            result_1: 10,
            result_2: 3544142,
        },
        InputResult {
            input: vec![1, 2, 3],
            result_1: 27,
            result_2: 261214,
        },
        InputResult {
            input: vec![2, 3, 1],
            result_1: 78,
            result_2: 6895259,
        },
        InputResult {
            input: vec![3, 2, 1],
            result_1: 438,
            result_2: 18,
        },
        InputResult {
            input: vec![3, 1, 2],
            result_1: 1836,
            result_2: 362,
        },
        InputResult {
            input: vec![14_usize, 8, 16, 0, 1, 17],
            result_1: 240,
            result_2: 505,
        },
    ];
    let run_size = 30000000;
    for ir in inputs.iter() {
        let InputResult {
            input,
            result_1,
            result_2,
        } = ir;
        println!("{:?}", input);
        let mut vec_try = vec![0_usize; run_size];
        let (last_spoken, elements) = input.split_last().unwrap();
        for (i, val) in elements.iter().enumerate() {
            vec_try[*val] = i + 1;
        }
        let mut last_spoken = *last_spoken;
        for turn in input.len()..run_size {
            let val = vec_try[last_spoken];
            if val == 0 {
                vec_try[last_spoken] = turn;
                last_spoken = 0;
            } else {
                let tmp = last_spoken;
                last_spoken = turn - val;
                vec_try[tmp] = turn;
            }
            if turn == 2020 - 1 {
                assert_eq!(last_spoken, *result_1);
            }
            if turn == 30000000 - 1 {
                assert_eq!(last_spoken, *result_2);
            }
        }
    }
}
