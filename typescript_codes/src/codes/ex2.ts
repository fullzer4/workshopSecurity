export const example2 = () => {
    let lista: Array<string | number> = ["item1", 2, "item"]

    lista.push("item2")
    console.log(lista)
    lista.push("item removivel")
    console.log(lista)
    lista.pop()
    console.log(lista)
}