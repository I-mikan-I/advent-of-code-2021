use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::rc::{Rc, Weak};

fn main() {
    let path = if let Some(x) = std::env::args().nth(1) {
        x
    } else {
        "2021/day-12/input-test".to_string()
    };
    let path = Path::new(&path);
    let contents = std::fs::read_to_string(path).unwrap();
    let (res_01, res_02) = challenge(&contents);
    println!(
        "Solution to part 1: {}!\n\
    Solution to part 2: {}!",
        res_01, res_02
    );
}

#[derive(Debug)]
struct Cave<'a> {
    name: &'a str,
    large: bool,
    next: Vec<Rc<RefCell<Cave<'a>>>>,
    previous: Vec<Weak<RefCell<Cave<'a>>>>,
}

impl<'a> Cave<'a> {
    fn new(s: &'a str) -> Result<Cave<'a>, ()> {
        let large = s.chars().all(char::is_uppercase);
        let next = Vec::new();
        let previous = Vec::new();
        let name = s.trim();
        Ok(Self {
            large,
            next,
            previous,
            name,
        })
    }
}

// This is not an optimal solution runtime wise. I decided to implement
// it as such (a linked data structure of caves, with a recursive algorithm),
// to get exposure to some of the more advanced smart pointer types.
fn challenge(input: &str) -> (usize, usize) {
    let mut lookup_table: HashMap<&str, Rc<RefCell<Cave>>> = HashMap::new();

    let map: Vec<[Cave; 2]> = input
        .lines()
        .map(|s| {
            s.split('-')
                .map(|c| Cave::new(c).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    for pair in map {
        let [start, end] = pair;
        let (key_s, key_e) = (start.name, end.name);
        let start = lookup_table
            .get(key_s)
            .cloned()
            .unwrap_or_else(|| Rc::new(RefCell::new(start)));
        let end = lookup_table
            .get(key_e)
            .cloned()
            .unwrap_or_else(|| Rc::new(RefCell::new(end)));
        start.borrow_mut().next.push(Rc::clone(&end));
        end.borrow_mut().previous.push(Rc::downgrade(&start));
        lookup_table.insert(key_s, start);
        lookup_table.insert(key_e, end);
    }

    let paths_1 = count_paths_1(lookup_table["start"].deref().borrow().deref());
    let paths_2 = count_paths_2(lookup_table["start"].deref().borrow().deref());
    (paths_1, paths_2)
}

fn count_paths_2(cave: &Cave) -> usize {
    count_paths_rec(cave, &[], false, None)
}

fn count_paths_1(cave: &Cave) -> usize {
    count_paths_rec(cave, &[], true, None)
}

fn count_paths_rec(
    cave: &Cave,
    visited: &[&str],
    visited_twice: bool,
    dual: Option<&str>,
) -> usize {
    let visited: Vec<&str> = visited
        .iter()
        .chain(std::iter::once(&cave.name))
        .copied()
        .collect();
    let visit_fn: Box<dyn Fn(Rc<RefCell<Cave>>) -> usize> =
        if visited_twice || cave.large || cave.name == "start" {
            Box::new(|cave: Rc<RefCell<Cave>>| {
                count_paths_rec(&(*cave).borrow(), &visited, visited_twice, dual)
            })
        } else {
            Box::new(|cave_: Rc<RefCell<Cave>>| {
                count_paths_rec(
                    &(*cave_).borrow(),
                    &visited[..visited.len() - 1],
                    true,
                    Some(cave.name),
                ) + count_paths_rec(&(*cave_).borrow(), &visited, visited_twice, dual)
            })
        };
    if cave.name == "end" {
        match dual {
            Some(dual) if visited.contains(&dual) => 1,
            None => 1,
            _ => 0,
        }
    } else {
        let prev_iter = cave.previous.iter().filter_map(Weak::upgrade);
        let to_visit = cave.next.iter().cloned().chain(prev_iter).filter(|cave| {
            (cave.deref()).borrow().large || !visited.contains(&cave.deref().borrow().name)
        });
        to_visit.map(visit_fn).sum()
    }
}
