/**
    Question: If you want to generalize the 
    largest(list: &[i32]) into different types,
    how will you do it?

**/
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}



fn main() {
    let v0 = vec![34, 50, 25, 100, 65];
    let v1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let v0_max = largest(&v0);
    let v1_max = largest(&v1);

    println!("v0_max = {}", v0_max);
    println!("v1_max = {}", v1_max);

}
