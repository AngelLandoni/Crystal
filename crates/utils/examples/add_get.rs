use utils::BlockVec;

fn main() {
    let mut vec = BlockVec::<String, 5>::new();
    vec.set("AAA".to_string(), 4);
    vec.set("AAA".to_string(), 8);
    vec.set("AAA".to_string(), 10);
    vec.set("AAA".to_string(), 15);
    vec.set("AAA".to_string(), 23);
    vec.set("AAA".to_string(), 66);
}