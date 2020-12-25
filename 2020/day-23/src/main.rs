use std::cell::{RefCell};
use std::rc::Rc;
use std::collections::HashMap;


fn get_result_1(head: Rc<RefCell<Node<i32>>>, game_len: usize) -> String {
    let mut temp_head = Rc::clone(&head);
    let mut val = temp_head.borrow().elem;
    while val != 1 {
        let abc = Rc::clone(&temp_head.borrow().next.as_ref().unwrap());
        temp_head = abc;
        val = temp_head.borrow().elem;
    }
    let abc = Rc::clone(&temp_head.borrow().next.as_ref().unwrap());
    temp_head = abc;
    let mut end_str = "".to_owned();
    for _ in 0..game_len - 1 {
        end_str.push_str(&temp_head.borrow().elem.to_string());
        let abc = Rc::clone(&temp_head.borrow().next.as_ref().unwrap());
        temp_head = abc;
    }
    end_str
}


fn get_result_2(head: Rc<RefCell<Node<i32>>>) -> usize {
    let mut temp_head = Rc::clone(&head);
    let mut val = temp_head.borrow().elem;
    while val != 1 {
        let abc = Rc::clone(&temp_head.borrow().next.as_ref().unwrap());
        temp_head = abc;
        val = temp_head.borrow().elem;
    }
    let next_elem = Rc::clone(&temp_head.borrow().next.as_ref().unwrap());
    let val_2 = next_elem.borrow().elem;
    let next_elem_2 = Rc::clone(&next_elem.borrow().next.as_ref().unwrap());
    val = next_elem_2.borrow().elem;
    (val as usize) * (val_2 as usize)
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

fn play_game_2(mut head: Rc<RefCell<Node<i32>>>, game_map: &HashMap<i32, Rc<RefCell<Node<i32>>>>, max_cup: i32, iterations: usize) -> Rc<RefCell<Node<i32>>> {
    for _ in 1..iterations+1 {
        let mut third_in_front = Rc::clone(&head);
        let first_of_three = Rc::clone(&head.borrow().next.as_ref().unwrap());
        for _ in 0..3 {
            let abc = Rc::clone(&third_in_front.borrow().next.as_ref().unwrap());
            third_in_front = abc;
        }
        let current_value = head.borrow().elem;
        let mut sought_value = current_value - 1;
        if sought_value == 0 {
            sought_value = max_cup;
        }
        let mut all_good = false;
        while !all_good {
            let mut tmp = Rc::clone(&head.borrow().next.as_ref().unwrap());
            all_good = true;
            for _ in 0..3 {
                if tmp.borrow().elem == sought_value {
                    sought_value -= 1;
                    if sought_value == 0 {
                        sought_value = max_cup;
                    }
                    all_good = false;
                    break;
                }
                let abc = Rc::clone(&tmp.borrow().next.as_ref().unwrap());
                tmp = abc;
            }
        }
        let new_tail = Rc::clone(game_map.get(&sought_value).unwrap());
        let temp_old_tail = Rc::clone(&new_tail.borrow().next.as_ref().unwrap());
        head.borrow_mut().next = Some(Rc::clone(&third_in_front.borrow().next.as_ref().unwrap()));
        new_tail.borrow_mut().next = Some(Rc::clone(&first_of_three));
        third_in_front.borrow_mut().next = Some(temp_old_tail);
        let abc = Rc::clone(&head.borrow().next.as_ref().unwrap());
        head = abc;
    }
    head
}

fn main() {
    let files_results = vec![
        ("389125467", "67384529", 149245887792_usize),
        ("315679824", "72496583", 41785843847_usize)
    ];
    for (input, result_1, result_2) in files_results.into_iter() {
        let game: Vec<i32> = input
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect();
        let (first, elements) = game.split_first().unwrap();
        let mut node = Rc::new(RefCell::new(Node{elem: *first, next: None}));
        let mut head = Rc::clone(&node);
        let max_cup = *game.iter().max().unwrap();

        for elem in elements.iter() {
            let second_node = Rc::new(RefCell::new(Node{elem: *elem, next:None}));
            node.borrow_mut().next = Some(second_node);
            let abc = Rc::clone(&node.borrow().next.as_ref().unwrap());
            node = abc;
        }
        node.borrow_mut().next = Some(Rc::clone(&head));

        let game_len = game.len();
        for _ in 0..game.len() {
            print!("{}", head.borrow().elem);
            let abc = Rc::clone(&head.borrow().next.as_ref().unwrap());
            head = abc;
        }
        let mut game_map: HashMap<i32, Rc<RefCell<Node<i32>>>> = HashMap::new();
        let mut tmp = Rc::clone(&head);
        for _ in 0..game.len() {
            let val = tmp.borrow().elem;
            let to_insert = Rc::clone(&tmp);
            game_map.insert(val, to_insert);
            let abc = Rc::clone(&tmp.borrow().next.as_ref().unwrap());
            tmp = abc;
        }
        println!("");
        head = play_game_2(head, &game_map, max_cup, 100);

        println!("PRINT GAME");
        for _ in 0..game.len() {
            print!("{}", head.borrow().elem);
            let abc = Rc::clone(&head.borrow().next.as_ref().unwrap());
            head = abc;
        }
        println!("");

        assert_eq!(get_result_1(Rc::clone(&head), game_len), result_1);

        let mut game_2: Vec<i32> = (1..=1_000_000).collect();
        for (i, el) in game.iter().enumerate() {
            game_2[i] = *el;
        }

        let (first, elements) = game_2.split_first().unwrap();
        let mut node = Rc::new(RefCell::new(Node{elem: *first, next: None}));
        let mut head = Rc::clone(&node);
        let max_cup = 1_000_000;

        for elem in elements.iter() {
            let second_node = Rc::new(RefCell::new(Node{elem: *elem, next:None}));
            node.borrow_mut().next = Some(second_node);
            let abc = Rc::clone(&node.borrow().next.as_ref().unwrap());
            node = abc;
        }
        node.borrow_mut().next = Some(Rc::clone(&head));

        let mut game_map: HashMap<i32, Rc<RefCell<Node<i32>>>> = HashMap::new();
        let mut tmp = Rc::clone(&head);
        for _ in 0..game_2.len() {
            let val = tmp.borrow().elem;
            let to_insert = Rc::clone(&tmp);
            game_map.insert(val, to_insert);
            let abc = Rc::clone(&tmp.borrow().next.as_ref().unwrap());
            tmp = abc;
        }

        head = play_game_2(head, &game_map, max_cup, 10_000_000);
        assert_eq!(get_result_2(Rc::clone(&head)), result_2);

    }
}
