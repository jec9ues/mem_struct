use mem_struct::data_struct::pos3::*;

#[test]
fn test_pos2() {
    let mut a = pos3(10.0, 0.0, 0.0);
    let mut b = pos3(100.0, 0.0, 0.0);

    println!("{:?}", a + b);
    println!("{:?}", a.distance(b));
    println!("{:?}", a[0]);
}