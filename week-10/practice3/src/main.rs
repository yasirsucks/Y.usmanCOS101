}

fn main(){

Let v = vec![20, 40, 60, 80];
// vector v owns the object in heap

Let v2 = v;
Let v2_return = display(v2);
println!("In main { :? }",v);

fn display(v:Vec<i32>)->Vec<i32> {
// returning same vector
println!("inside display { :? }",v);
return v;
}