DATATYPES..................

fn main() {
    
    println!("Hello, world!");
    let ans:u32 = sum(32 ,3);

    println!("{}" ,ans );

    let name = String::from("shanxar");
    println!("cool name is {}", name);

    let v = vec![1,2,3,4,5];

    println!("{:?}" , v);

    for i in 0..100{
        print!("{} ",i);
    }
}


fn sum(a:u32 , b:u32) -> u32{

return a+b;

}


// MEMORY MANAGEMENT AND BORROWING.............

// fn main(){

// let mut s1 = String::from("hello");
// prin(&mut s1);
// println!("{}",s1)

// }

// fn prin(tem: &mut String){

//    tem.push_str(" rust");

// }



// STRUCT...................................

// struct Rect{

//    height: u32,
//    width: u32,
// }

// impl Rect{

// fn area(&self)->u32{
//    self.height*self.width
// }

// fn perimeter(&self)->u32{
//    2*(self.height+self.width)
// }


// }
// fn main(){

//   let rect1 = Rect{
//    height : 10,
//    width:5,
//   };

//    println!("area of rectangle is {}", rect1.area());
//    println!("perimeter of rectangle is {}", rect1.perimeter());

// }

// ENUM.........................

// enum Shape {
//    Circle(f64),
//    Rectangle(f64,f64),
// }

// fn main(){

// let my_shape = Shape::Rectangle(10.0,5.0);
// area(my_shape);



// }
// fn area(shape:Shape)->f64{

//   let ans = match shape{
//       Shape::Rectangle(a,b)=>a*b,
//       Shape::Circle(r)=>3.14*r*r,
//    };
//    return ans;
// }

// Option and Result ENUM..................

// Option is used for None.
// Result is used for error handling.

// Option....

// fn main(){

//    let index = get_index_of(String::from("shanxar"));

//    match index{

//       Some(index)=> println!("found index of a at {}" , index),
//       None=> println!("no index found"),

//    }
  
// }

// fn get_index_of(fullstr: String) -> Option<i32>{

// for(index , char) in fullstr.chars().enumerate(){

//    if char == 'a'{

//    return Some(index as i32);
   

//    }
// }

// return None;
// }

// Result................


// use std::fs::read_to_string;

// fn main(){

// read_file_fn(String::from("a.txt"));

// }

// fn read_file_fn(filepath: String){

// let fil = read_to_string(filepath);

// match fil {

// Ok(data)=>println!("here is data {}", data),
// Err(error)=>println!("got error: {}", error),

// }



// }



// COLLECTION..........................


// VECTOR......................

// fn main(){

//     let mut vec = Vec::new();

//     vec.push(1);
//     vec.push(3);
//     vec.push(7);
//     vec.push(4);
//     vec.push(10);



//     println!("{:?}", vec);
//    println!("even vector : {:?} ",even_vector(vec));
// }


// fn even_vector(inp : Vec<i32>)-> Vec<i32>{

//     let mut even_vec = Vec::new();
// for val in inp{

// if val%2 == 0 {
//     even_vec.push(val);
// }

// }

// return even_vec;


// }

// HASHMAP...........................

// use std::collections::HashMap;

// fn main(){

// let  bio = vec![(String::from("shanxar"), 20), (String::from("Alle_Doggo"),9)];

// println!("{:?}", convert_vec_into_hashmap(bio));


// }

// fn convert_vec_into_hashmap(vec : Vec<(String,u32)>)->HashMap<String,u32>{

//     let mut map = HashMap::new();
// for(name,age) in vec {
    
//     map.insert(name,age);
// }

// return map;
// }
