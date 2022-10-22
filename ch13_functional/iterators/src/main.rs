#[allow(dead_code)]
fn sample_iter_v1() {
    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();
   
    // To access the iterator for loops, you need to invoke v1.iter()
    println!("v1_iter = {:?}", v1_iter);

    for val in v1 {
        println!("val = {}", val);
    }
}

// Using next in iterators
#[allow(dead_code)]
fn sample_iter_v2() {
    let v1 = vec![1, 2, 3, 4, 5];
    
    // In order to save the state of current iterator
    // v1_iter needs to be mutable
    // so whenever you try to invoke v1_iter.next()
    // v1_iter can internally save the current iterator
    // state.
    let mut v1_iter = v1.iter();

    let v1_next = v1_iter.next();
    let v2_next = v1_iter.next();
    let v3_next = v1_iter.next();
    let v4_next = v1_iter.next();
    let v5_next = v1_iter.next();
    let v6_next = v1_iter.next();

    println!("v1_next = {:?}", v1_next);
    println!("v2_next = {:?}", v2_next);
    println!("v3_next = {:?}", v3_next);
    println!("v4_next = {:?}", v4_next);
    println!("v5_next = {:?}", v5_next);
    println!("v6_next = {:?}", v6_next);
}

// Using iterator.sum()
#[allow(dead_code)]
fn sample_iter_v3() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    println!("total = {}", total);
    
}

struct MutabilityOfMembers {
    num_list: Vec<u32>, 
}

impl MutabilityOfMembers {
    fn sum_first_three(&self) -> u32 {
        let mut total_sum: u32 = 0;
        let mut num_list_iter = self.num_list.iter();

        total_sum += num_list_iter.next().unwrap();
        total_sum += num_list_iter.next().unwrap();
        total_sum += num_list_iter.next().unwrap();
        
        total_sum
    }
}

// This is for curiosity purposes
#[allow(dead_code)]
fn sample_iter_v4() {

    let mom = MutabilityOfMembers {
        num_list: vec![1, 2, 3, 4, 5],
    };

    println!("mom_sum = {}", mom.sum_first_three());
}


// Iterator Adaptors
#[allow(dead_code)]
fn sample_iter_v5() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2 = v1.iter().map(|x| x * x);

    for val in v2 {
        println!("val = {}", val);
    }
}

// Iterator Adaptors that stops lazy execution
// This is beautiful
#[allow(dead_code)]
fn sample_iter_v6() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v2: Vec<_> = v1.iter().map(|x| x * x).collect();

    println!("v2 = {:?}", v2);
}

// Using filters v1
// This looks nice
#[test]
fn sample_iter_v7() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9 , 10];
    let filter_val = 6;

    let v2: Vec<i32> = v1.into_iter().filter(|v| v < &filter_val).collect();

    assert_eq!(v2, vec![1, 2, 3, 4, 5]);
}


fn main() {
    sample_iter_v6();
}
