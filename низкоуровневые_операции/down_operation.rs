use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    let story = String::from("Rust By Practice");
    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);
    let ptr = story.as_mut_ptr();

    let len = story.len();
    let cap = story.capacity();
    assert_eq!(len, 16);

    let s = unsafe { String::from_raw_parts(ptr, len, cap) };
    assert_eq!(s, *story);

    println!("Success!");
}