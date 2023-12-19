

fn main() {
    let _a = [1, 2, 3, 4, 5];

    // let v: Vec<i32> = a.iter().map(|x| x * 2).collect();
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(6);
    v.push(9);
    println!("{:?}", v);

    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);




    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("{}", third);

    match v.get(20) {
        Some(third) => println!("{}", third),
        None => println!("There is no third element."),
    }

    
}
