use std::collections::VecDeque;

fn highest_score(n_players: u32, last_pt: u32) -> u32
{
    let mut scores = vec![0u32; n_players as usize]; 
    let mut circle = VecDeque::new();

    circle.push_back(0);
    for i in 1..=last_pt {
        if i % 23 == 0 {
            circle.rotate_right(7);
            scores[(i % n_players) as usize] += i + circle.pop_front().unwrap();
        }
        else {
            circle.rotate_left(2 % circle.len());
            circle.push_front(i);
        }
    }
    return *scores.iter().max().unwrap();
}

fn main()
{
    assert_eq!(highest_score(10, 1618), 8317);
    assert_eq!(highest_score(13, 7999), 146373);
    assert_eq!(highest_score(17, 1104), 2764);
    assert_eq!(highest_score(21, 6111), 54718);
    assert_eq!(highest_score(30, 5807), 37305);

    println!("Part 1: {}", highest_score(419, 71052));
    println!("Part 2: {}", highest_score(419, 7105200));
}