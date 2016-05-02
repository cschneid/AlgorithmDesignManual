fn main() {
    let data: Vec<_> = "INSERTIONSORT".to_owned().chars().collect();
    let result = insertion_sort(data.clone());
    print!("{:?} sorted is {:?}", data, result);

    let data2: Vec<_> = "ABCDABCDDCBADCBA".to_owned().chars().collect();
    let result2 = insertion_sort(data2.clone());
    print!("{:?} sorted is {:?}", data2, result2);
}

fn insertion_sort(input: Vec<char>) -> Vec<char> {
    let mut result = Vec::with_capacity(input.len());

    // For each letter in the input
    for c in input.iter() {
        let mut pushAt = None;

        // Find where in the result it should go.
        // We build up the result as we go.
        for (i, rc) in result.iter().enumerate() {
            if rc > c {
                print!("{:?} is less than {:?}, pushing at {:?}\n", rc, c, i);
                pushAt = Some(i);
                break
            }
        }

        match pushAt {
            Some(i) => result.insert(i, *c),
            None => result.push(*c)
        }
    }

    return result
}
