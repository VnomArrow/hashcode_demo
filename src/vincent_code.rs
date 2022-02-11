#[derive(Debug, Default)]
pub struct Person {
    pub likes : Vec<usize>,
    pub dislikes : Vec<usize>,
}

#[inline]
pub fn is_valid(person : &Person, pizza: &Vec<usize>) -> bool {
    for item in &person.likes {
        if !pizza.contains(item)  {
            return false;
        }
    }

    for item in pizza {
        if person.dislikes.contains(item)  {
            return false
        }
    }

    true
}

pub fn solve(list : &Vec<Person>, pizza: &Vec<usize>) -> usize {
    let mut valid_ppl = 0usize;
    for person in list {
        if is_valid(person, pizza) {
            valid_ppl += 1;
        }
    }
    valid_ppl
}