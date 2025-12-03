fn main(){

Let v = vec![10,20,30];
// vector v owns the object in heap

Let v2 = v;

display(v2);
// v2 is moved to display and v2 is invalidated

println!("In main { :? }",v2);
//v2 is No longer usable here

// moves ownership to v2

}

fn display(v:Vec<i32>){
println!("inside display { :? }", v);

}