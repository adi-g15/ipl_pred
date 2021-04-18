use std::cmp::Ord;

pub fn max_element<T: Copy+Ord>(vec: &[T]) -> (T,usize) {
    if vec.len() == 0 {
        panic!("Empty slice/vector/array");
    }

    let (mut max,mut max_i) = (vec[0], 0);
    let mut index = 0usize;
    for i in vec {
        if max < *i {
            max = *i;
            max_i = index;
        }

        index += 1;
    }

    (max,max_i)
}
