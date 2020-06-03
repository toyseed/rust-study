#[test]
fn test_vec() {
    let mut v = vec![1, 2, 3, 4];

    let does_not_exist = v.get(100);

    let mut first = &mut v[0];
    *first += 10;

    println!("{:?}", v);

    for i in &mut v {
        *i += 10;
    }

    println!("{:?}", v);
}

/*
adding a new element onto the end of the vector might require allocating new memory
and copying the old elements to the new space,
if there isn’t enough room to put all the elements next to each other
where the vector currently is
 */
