// DATATYPES..................

// fn main() {
    
//     println!("Hello, world!");
//     let ans:u32 = sum(32 ,3);

//     println!("{}" ,ans );

//     let name = String::from("shanxar");
//     println!("cool name is {}", name);

//     let v = vec![1,2,3,4,5];

//     println!("{:?}" , v);

//     for i in 0..100{
//         print!("{} ",i);
//     }
// }


// fn sum(a:u32 , b:u32) -> u32{

// return a+b;

// }


// MEMORY MANAGEMENT AND BORROWING.............

// fn main(){

// let mut s1 = String::from("hello");
// prin(&mut s1);
// println!("{}",s1)

// }

// fn prin(tem: &mut String){

//    tem.push_str(" rust");

// }



// STRUCT...............................................................................

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

// ENUM......................................................................

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

// Option and Result ENUM..............................................................................

// Option is used for None.
// Result is used for error handling.

// Option................................................

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

// Result.................................................


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

// ITERATOR.......................................

// => iter()  immutable, Borrow , Just read values
// => iter_mut() mutable, Borrow , change values
// => into_iter() mutable, move ownership , used in regular for loop by default


// assignment : filter odd values , then double them and create a new vector.
// fn main(){

//     let vec = vec![1,2,3,4,7];
   
//     let iter = vec.iter();
    

//     let iter_ans = iter.filter(|x| *x%2 != 0).map(|x| x*2);

    //  here we create a new vector
    //  let mut res = Vec::new();
    // for value in iter_ans {
    //     res.push(value);
    // }

    // here we change a iterator back into a vector using collect();
//     let res : Vec<i32> = iter_ans.collect();
 
//     for i in res {
//         println!("{} ", i);
//     }


// }


// STRING SLICES..............................................

// fn main(){

// let name = String::from("shankar singh");
// let first_n = first_name(&name);
// println!("{}" , first_n);


// }
// fn first_name(str : &String) -> &str {

// let mut index = 0;

// for (_ , i) in str.chars().enumerate(){
    
    
//     if i == ' ' {
//         break;
//     }
//     index += 1;
// }

// return &str[0..=index];

// }

//GENERIC...................................................................


// fn main(){

// println!("largest number {}", largest(3,6));
// println!("largest char {}", largest("f","a"));


// }

// fn largest<T:std::cmp::PartialOrd>(a:T,b:T) -> T{

//     if a > b {
//             a
//     } else {
//             b
//     }
// }



// TRAITS = it's lika abstract class..............................................................

// pub trait Summary{
// fn summarize(&self)-> String{
//     return String::from("hello this summary");
// }
// }

// trait Fix {
// fn fix(&self)-> String{
//     return String::from("hello this is fix");
// }
// }
// struct User {
//     name : String,
//     age : u32,
// }

// impl Summary for User{
//     fn summarize(&self)-> String{
//         return format!("name is {} , age is {}", self.name , self.age);  // overriding of summarize()
//     }
// }

// impl Fix for User{
    
// }


// fn main(){

//     let user1 = User{
//         name : String::from("shanxar"),
//         age : 21,
//     };
 
//  notify(user1);
// //  println!("{}", user1.summarize());
// }

// // MULTIPLE TRAIT BOUND OR TRAIT AS A FN PARAMETER 
// // LIKE A FN WILL ONLY IMPLEMENT IF IT'S IMP CERTAIN TRAIT

// fn notify<T: Summary + Fix>(u:T){

// println!("{}", u.summarize());
// println!("{}", u.fix());

// }


// //LIFETIME........................................................................................................

// fn main(){
// let longest_str;
// let str1 = String::from("fsd");
//          {
//                let str2 = String::from("fsdfdfafdf");
//                longest_str=largest(&str1,&str2);

//          }
//          println!("{}", longest_str); // longest_str is only valid in that block for correct ans.
// }

// fn largest<'a>(fir:&'a str , b:&'a str)-> &'a str{  // 'a is the lifetime generic

// if fir.len() > b.len() {
//     fir
// } else {
//     b
// }

// }

// //STRUCT WITH LIFETIMES.......................................

// struct User<'a>{ // this specifies that if name goes out of scope then User will ultimately go out of scope
//             name : &'a str, 
// }

// fn main(){

//     let nam = String::from("shanxar");
//     let user1 = User{
//             name: &nam,
//     };

//     println!("{}", user1.name);
// }

// // MULTITHREADING.........................................................................................................

// // MOVE KEYWORD : USED TO move ownership in a thread....................................................................



// use std::thread;

// fn main(){

// let v = vec![2,3,4];

//    let handle = thread::spawn( move|| {

// println!("vector is {:?}", v)

//     });

//     handle.join().unwrap();

// }


// // MESSAGE PASSING...........................................................................................

// use std::{
//     sync::mpsc,
//     thread,
// };

// fn main(){

// let (tx,rx) = mpsc::channel();

// thread::spawn(move || {

//       tx.send(String::from("shanxarr")).unwrap();

// });

// let value = rx.recv();  // not using unwrap() coz can cause panic in thread or stop execution

// match value {
//     Ok(value)=> println!("{}", value),
//     Err(error)=> println!("got error in reading value {}", error),
// }

// }

// // PROGRAM TO FIND SUM OF 1-10^8
use std::{
  sync::mpsc,
   thread};

fn main() {
let (tx, rx) = mpsc::channel();


let tx1 = tx.clone();
let handle1 = thread::spawn(move || {
    let mut ans:u128 = 0;
    for i in 1..100 {
        ans += i;
    }
    tx1.send(ans).unwrap();
});

let tx2 = tx.clone();
let handle2 = thread::spawn(move || {
    let mut ans:u128 = 0;
    for i in 100..10000 {
        ans += i;
    }
    tx2.send(ans).unwrap();
});

let tx3 = tx.clone();
let handle3 = thread::spawn(move || {
    let mut ans:u128 = 0;
    for i in 10000..1000000 {
        ans += i;
    }
    tx3.send(ans).unwrap();
});

handle1.join().unwrap();
handle2.join().unwrap();
handle3.join().unwrap();

let mut total:u128 = 0;
for _ in 0..3 {
    total += rx.recv().unwrap();
}

for i in 1000000..=10000000 {
    total += i;
}

println!("sum of 1-10^8 = {}", total);


}
