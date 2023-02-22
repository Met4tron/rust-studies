use rand::Rng;

fn main () {
  // without type declaration, inference
  let arr = [1,2,3,4,5];
  println!("The array is {:?}", arr);
  println!("The length of the array is: {}", arr.len());

  println!("\n");

  // with type and size determined
  let arr2:[i32;5] = [1,2,3,4,5];
  println!("The array is {:?}", arr2);
  println!("Length of the array is: {}", arr2.len());

  println!("\n");


  // default values, types and size
  let arr3:[i32;3] = [0;3];// 0 => value, 3 => size
  println!("The array is: {:?}", arr3);
  println!("Length of the array is: {}", arr3.len());

  println!("\n");


  // iterations

  // for loop using size of array
  for i in 0..arr3.len() {
    println!("{} -> {}", i, arr3[i]);
  }

  println!("\n");

  // using std iter()
  for (element, index) in arr3.iter().enumerate() {
    println!("{} -> {}", index, element);
  }

  println!("\n");

  // filter
  let mut rng = rand::thread_rng();
  let arr4: [i32; 20] = rng.gen();

  let oddValues: Vec<_> = arr4.iter().filter(|&e| e % 2 == 0).collect();
  let evenValues: Vec<_> = arr4.iter().filter(|&e| e % 2 != 0).collect();

  println!("Odd => {:?} \nEven => {:?}", oddValues, evenValues)
 
}