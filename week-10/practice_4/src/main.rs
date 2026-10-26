fn main(){
    // a list of nos
    let v = vec![15, 25, 35, 45, 55];
    print_vector(v);
    // println!("{}", v[0]); // This line gives an error because v was moved into print_vector
}

fn print_vector(x: Vec<i32>){
    println!("Inside print_vector function {:?}", x);
}