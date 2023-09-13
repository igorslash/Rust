fn main() {
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let c = [3; 5];//[3, 3, 3, 3, 3]

    a = [1];
    println!("{:?}", {a});

    let array = [1, 2, 3, 4, 4, 5, 6, 6];
    let mut i = 0;
    while i < array.len() {
        let mut j = i;
        while j < array.len() {
            println!("{} {}", array[i], array[j]);
        }
        j += 1
    }
    i += 1;
}