struct FilterCondition {
    item: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        item == &self.item
    }
}
fn custom_filter(collection: &Vec<i32>, filter_struct: &FilterCondition) -> Vec<i32> {
    let mut filtered_collection: Vec<i32> = Vec::new();
    for item in collection.iter() {
        if filter_struct.is_match(&item) {
            filtered_collection.push(*item); //dereference pushed values 
        }
    }
    filtered_collection
}

fn main() {
    let filter_struct = FilterCondition { item: 4 };
    let collection = vec![1, 2, 3, 4, 5];
    // without closures
    let filtered = custom_filter(&collection, &filter_struct);
    println!("{:?}", filtered);
    // using closures
    let filtered: Vec<i32> = collection.into_iter().filter(|x| *x == 4).collect(); // dereference x 
    println!("{:?}", filtered);
}
