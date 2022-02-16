use std::collections::HashMap;
use std::fs::File;
use crate::util::*;


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Ingrediant {
    pub name: String,
    pub occurance_count: usize,
    pub liked_persons: Vec<usize>,
    pub disliked_persons: Vec<usize>,
}

// Used to count the occurances of an ingredient in users preferences
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Occurance {
    pub count: usize,
    pub index: usize
}

#[derive(Debug, Default)]
pub struct PizzaPreferenceInfo {
    pub persons: Vec<Person>,
    pub ingrediant_name_to_index_hashmap: HashMap<String, usize>, // Returns the index in the ingrediants
    pub ingrediants: Vec<Ingrediant>
}
impl PizzaPreferenceInfo {
    // Adds occurance for ingrediant and makes sure it exists
    // Returns the ingrediant index
    fn add_ingrediant_occurance(&mut self, ingrediant_name: &str) -> usize {
        // Try to fetch the ingrediants index
        let ingrediant_index_option = self.ingrediant_name_to_index_hashmap.get(ingrediant_name);
        let ingrediant_index: usize;
        if ingrediant_index_option.is_some() {
            // The ingrediant exists
            ingrediant_index = *ingrediant_index_option.unwrap();
        }
        else {
            // The ingreadiant does not exist yet and must be created
            ingrediant_index = self.ingrediants.len();

            self.ingrediants.push(Ingrediant {
                name: ingrediant_name.to_string(),
                occurance_count: 0,
                liked_persons: vec!(),
                disliked_persons: vec!()
            });

            self.ingrediant_name_to_index_hashmap.insert(ingrediant_name.to_string(), ingrediant_index);
        }

        // Add an occurance for ingrediant
        self.ingrediants[ingrediant_index].occurance_count += 1;
        return ingrediant_index;
    }
    pub fn add_liked_ingrediant(&mut self, ingrediant_name: &str, person_index: usize) {
        let ingrediant_index = self.add_ingrediant_occurance(ingrediant_name);

        // Make ingrediant know that the person likes it
        self.ingrediants[ingrediant_index].liked_persons.push(person_index);

        // Make person know it likes the ingrediant
        self.persons[person_index].likes.push(ingrediant_index)
    }

    pub fn add_disliked_ingrediant(&mut self, ingrediant_name: &str, person_index: usize) {
        let ingrediant_index = self.add_ingrediant_occurance(ingrediant_name);
        
        // Make ingrediant know that the person dislikes it
        self.ingrediants[ingrediant_index].disliked_persons.push(person_index);

        // Make person know it dislikes the ingrediant
        self.persons[person_index].dislikes.push(ingrediant_index)
    }
    pub fn list_of_occurences(&self) -> Vec<Occurance> {
        // Crate the list and list all of the ingredients and their occurance
        let mut occurance_list: Vec<Occurance> = vec!();
        occurance_list.reserve(self.ingrediants.len());
        for i in 0..self.ingrediants.len() {
            occurance_list.push(Occurance{count: self.ingrediants[i].occurance_count, index: i})
        }

        // Sort the list by occurances, biggest first
        occurance_list.sort_by(|a, b| b.count.cmp(&a.count));

        return occurance_list;
    }
}

// Parse the file of ingredient preferences
pub fn parse_preference_info(file: &str) -> PizzaPreferenceInfo {
    let mut preference_info: PizzaPreferenceInfo = Default::default();

    let data = &std::fs::read(file).unwrap();
    let mut lines = std::str::from_utf8(data).unwrap().lines();

    let ppl = crate::util::read::<usize>(lines.next().unwrap());
    for _ in 0..ppl {
        preference_info.persons.push(Default::default());

        let like = lines.next().unwrap();
        let mut line_split = like.split_whitespace();
        line_split.next();

        for i in line_split {
            preference_info.add_liked_ingrediant(i, preference_info.persons.len() - 1);
        }
        let dislike = lines.next().unwrap();

        let mut line_split = dislike.split_whitespace();
        line_split.next();
        for i in line_split {
            preference_info.add_disliked_ingrediant(i, preference_info.persons.len() - 1);
        }
    }
    return preference_info;
}

#[derive(Debug, Default)]
pub struct Person {
    pub likes : Vec<usize>,
    pub dislikes : Vec<usize>,
}