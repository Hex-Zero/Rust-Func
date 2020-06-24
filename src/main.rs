fn main() {
    println!("start");
    let mut prime = [true; 30000];

    let mut p = 2;
    while p * p < 30000 {
        if prime[p] {
            let mut i = p * p;
            while i < 30000 {
                prime[i] = false;
                i += p
            }
        }
        p += 1;
    }
    let mut index = 0;
    for item in prime.iter() {
        println!("{}:{}", index, item);
        index += 1;
    }
}
