fn dump(arr: &[i32]){
        println!("arr is {:?}", arr);
}

fn main(){
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);
    println!("first {:?}", first);
    println!("last {:?}", last);

    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];
    let maybe_first = v.get(2);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    dump(&v2);

    let slice = &v2[1..];
    println!("slice is {:?}", slice);

    let mut iter = 0..3;
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

}