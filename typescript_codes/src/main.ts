import { example2 } from './codes/ex2';
import { example1 } from './codes/ex1';
const iniciate = (task: number) => {
    
    if(task === 1){
        example1()
    }else if(task === 2){
        example2()
    }
}

iniciate(2)