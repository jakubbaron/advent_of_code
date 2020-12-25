use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

fn get_sought_value(node: Rc<RefCell<Node<i32>>>, reset_cup: i32, max_cup: i32) -> i32 {
    let current_value = node.borrow().elem;
    let mut sought_value = current_value - 1;
    if sought_value == reset_cup {
        sought_value = max_cup;
    }
    let mut all_good = false;
    while !all_good {
        let mut tmp = node.borrow().get_next();
        all_good = true;
        for _ in 0..3 {
            if tmp.borrow().elem == sought_value {
                sought_value = sought_value - 1;
                if sought_value == reset_cup {
                    sought_value = max_cup;
                }
                all_good = false;
                break;
            }
            tmp = get_next_rc(&tmp);
        }
    }
    sought_value
}
fn play_game_2(
    mut head: Rc<RefCell<Node<i32>>>,
    game_map: &HashMap<i32, Rc<RefCell<Node<i32>>>>,
    max_cup: i32,
    iterations: usize,
) -> Rc<RefCell<Node<i32>>> {
    for _ in 1..iterations + 1 {
        let first_of_three = Rc::clone(&head.borrow().next.as_ref().unwrap());
        let mut third_in_front = Rc::clone(&head);
        for _ in 0..3 {
            third_in_front = get_next_rc(&third_in_front);
        }
        let sought_value = get_sought_value(Rc::clone(&head), 0, max_cup);
        let new_tail = Rc::clone(game_map.get(&sought_value).unwrap());
        let temp_old_tail = get_next_rc(&new_tail);
        head.borrow_mut().set_next_rc(get_next_rc(&third_in_front));
        new_tail.borrow_mut().set_next_rc(first_of_three);
        third_in_front.borrow_mut().next = Some(temp_old_tail);
        head = get_next_rc(&head);
    }
    head
}

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node{elem, next:None}
    }

    fn set_next(&mut self, node: Node<T>) {
        self.next = Some(Rc::new(RefCell::new(node)));
    }
    fn set_next_rc(&mut self, node_rc: Rc<RefCell<Node<T>>>) {
        self.next = Some(node_rc);
    }

    fn get_next(&self) -> Rc<RefCell<Node<T>>> {
        Rc::clone(&self.next.as_ref().unwrap())
    }
}

fn get_next_rc<T>(node: &Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    let tmp = node.borrow().get_next();
    tmp
}

fn print_nodes<T: std::fmt::Display> (mut node: Rc<RefCell<Node<T>>>, len: usize, curr_id: usize) {
    for i in 0..len {
        if i != curr_id {
            print!("{} ", node.borrow().elem);
        } else {
            print!("({}) ", node.borrow().elem);
        }
        node = get_next_rc(&node);
    }
    println!("");
}

fn create_map<T: Eq + std::hash::Hash + Copy> (mut node: Rc<RefCell<Node<T>>>, len: usize) -> HashMap<T, Rc<RefCell<Node<T>>>> {
    let mut game_map = HashMap::new();
    for _ in 0..len {
        let val = node.borrow().elem;
        game_map.insert(val, Rc::clone(&node));
        node = get_next_rc(&node);
    }
    game_map
}

fn from_vec<T: Copy>(v: &[T]) -> (Rc<RefCell<Node<T>>>, Rc<RefCell<Node<T>>>) {
    let mut node = Rc::new(RefCell::new(Node::new(v[0])));
    let head = Rc::clone(&node);
    let elements = &v[1..v.len()];
    for elem in elements.iter() {
        node.borrow_mut().set_next(Node::new(*elem));
        node = get_next_rc(&node);
    }
    (node, head)
}

fn main() {
    let files_results = vec![
        ("389125467", "67384529", 149245887792_usize),
        ("315679824", "72496583", 41785843847_usize),
    ];
    for (input, result_1, result_2) in files_results.into_iter() {
        let game: Vec<i32> = input
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect();
        let max_cup = *game.iter().max().unwrap();

        let (node, mut head) = from_vec(&game);
        node.borrow_mut().next = Some(Rc::clone(&head));

        let game_len = game.len();
        print_nodes(Rc::clone(&head), game_len, 0);

        let game_map = create_map::<i32>(Rc::clone(&head), game_len);
        head = play_game_2(head, &game_map, max_cup, 100);
        assert_eq!(get_result_1(Rc::clone(&head), game_len), result_1);

        let mut game_2: Vec<i32> = (1..=1_000_000).collect();
        let max_cup = 1_000_000;
        for (i, el) in game.iter().enumerate() {
            game_2[i] = *el;
        }

        let (node, mut head) = from_vec(&game_2);
        node.borrow_mut().next = Some(Rc::clone(&head));

        let game_len = game_2.len();
        let game_map = create_map::<i32>(Rc::clone(&head), game_len);
        head = play_game_2(head, &game_map, max_cup, 10_000_000);
        assert_eq!(get_result_2(Rc::clone(&head)), result_2);
    }
}
