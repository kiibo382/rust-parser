use std::rc::Rc;

fn main() {
    let mut s: String = "sirataki".to_string();
    let t = &s;
    // *t = "harusame".to_string();
    s = "harusame".to_string();
    // println!("{}", t); // error!
}
