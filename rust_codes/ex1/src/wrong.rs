fn printar(item: Vec<u32>){
    for i in item {
        println!("{}", i);
    }
}

fn main() {
    let item = vec![1,2,3];

    printar(item);
    printar(item);
}