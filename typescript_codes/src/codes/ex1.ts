export const example1 = () => {
    let item: Array<number> = [1,2,3]

    const loop = (item: Array<number>) => {
        for (const i of item){
            console.log(i);
        }
    }

    loop(item)
    loop(item)
}