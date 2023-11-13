use mem_strcut::data_struct::pos2::*;

#[test]
fn test_pos2() {
    let mut a = pos2(10.0, 0.0);
    let mut b = pos2(0.0, 10.0);

    println!("{:?}", a + b);
    println!("{:?}", a.distance(b));
}