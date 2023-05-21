fn main(){
    let mut lista = vec!<&str | int32>["item1", 2, "item"];
    // nao tem como criar um vector de dois tipos
    lista.push("item2");
    println!("{:?}", lista);
    lista.push("item removivel");
    println!("{:?}", lista);
    lista.pop();
    println!("{:?}", lista);
}