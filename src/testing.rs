pub fn test_iter() {
    let a = ['a', 'b', 'c'];

    let mut iter = a.iter().enumerate();

    for i in iter {
        println!("{:?}", i);
    }
}
