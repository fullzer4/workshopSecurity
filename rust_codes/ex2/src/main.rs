#[derive(Debug)]

enum Tipo {
    String(String),
    Int(i32),
}
    
fn main() {
    let mut lista: Vec<Tipo> = vec![
        Tipo::String("item1".to_string()),
        Tipo::Int(2),
        Tipo::String("item".to_string()),
    ];
    
    lista.push(Tipo::String("item2".to_string()));
    println!("{:?}", lista);

    lista.push(Tipo::String("item removivel".to_string()));
    println!("{:?}", lista);

    lista.pop();
    println!("{:?}", lista);
}
