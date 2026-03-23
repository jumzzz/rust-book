struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop Invokes...");
    }
}


fn main() {
    let c0 = CustomSmartPointer {
        data: String::from("yes"),
    };
    
    let c1 = c0;

    let c2 = c1; 
    let c3 = c2;
    let c4 = c3;

}
