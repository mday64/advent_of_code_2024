use ndarray::{arr1, arr2, s};

fn main() {
    let m = arr2(&[[1, 2, 3, 4, 5],
                   [6, 7, 8, 9, 10],
                   [11, 12, 13, 14, 15]]);

    assert_eq!(m[(1, 3)], 9);
    
    let s0 = m.slice(s![1, 2..]);
    let s1 = arr1(&[8, 9, 10]);
    assert_eq!(s0, s1);
    
    assert_eq!(m.slice(s![1, 0..3;-1]), arr1(&[8, 7, 6]));
    
    for elem in s0.iter() {
        println!("{elem}");
    }
}
