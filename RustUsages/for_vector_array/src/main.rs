fn main() {
    let array = [
        1, 2, 3, 4, 5, 6, 7, 8, 9,
        10, 20, 47, 59, 63, 75, 88,
        99, 107, 120, 133, 155, 162,
        176,188, 199, 200, 210, 222
    ];
    let array2 = [0;10];
    println!("{}",array2[4]);
    println!("size of the array2: {}",array2.len());

    let var = array[0];
    println!("{}", array[4]);
    println!("{}",var);
    let mut array3 : Vec<i64> = Vec:: new(); //defining mutable vector 

    //for loop works 500 times 
    for i in 0..500{
        array3.push(i);                 //every step we push the sequential numbers to array3
        let index = i as usize;         //changing the type of the i
        println!("{}", array3[index]);  //after push printing the array element
    }

    println!("--------------");
    //The for loop will run up to the number of elements of the array
    for i in array{
        println!("{}", i); //printing all array elements
    }

}