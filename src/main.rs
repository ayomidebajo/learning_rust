// fn convert_celsius_to_fahrenheit(x: i32) -> f32 {
//     let deci = 1.8;
// let result = x as f32 * deci;
// return 32 as f32 + result;
// } 

// fn convert_fahrenheit_to_celsius(x:i32) -> f32 {
//     let deci = 0.56;
// let result = x as f32 - 32 as f32;
// return result * deci
// }

// fn fibonacci_func(arr:[i32; 7], x: i32) {
//     let mut y = arr[0];
//     let mut index = 0;
// for item in arr.iter() {

//      y += item;
//     println!("item {}", item);
//     println!("y {}", y);
//     println!("{}", x);
//     if index == x {
//         println!("item at index {}", item);
//         println!("the y {}", y)
//           }
// index += 1
// };
// }

fn twelve_days_of_christmas () {
    let songs = ["A patridge in a pear tree", "Two turtle doves and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let owned_string: &str = "On the "; 
    let day_christmas_love: &str = " day of christmas my true love gave to me";
    println!("ALERT!!! {}", owned_string);
    for item in days {     
        if item == "first" {          
            let mut index = 1;          
            let owned_string = String::from("On the ");                    
            let together = owned_string + item + day_christmas_love;
            println!("{:?}", together);
        loop {             
               index -= 1;
             println!("{}", songs[index]);     
          if index <= 0 {
              break;
          };
        };
        }  else if item == "second" {
            let mut index = 2;          
            let owned_string = String::from("On the ");                     
            let together = owned_string + item + day_christmas_love;
            println!("{:?}", together);
           loop {             
               index -= 1;
             println!("{}", songs[index]);     
          if index <= 0 {
              break;
          };
        };
        } else if item == "third" {
            let mut index = 3;          
            let owned_string = String::from("On the ");                    
            let together = owned_string + item + day_christmas_love;
            println!("{:?}", together);
           loop {             
               index -= 1;
             println!("{}", songs[index]);     
          if index <= 0 {
              break;
          };
        };
        }  else if item == "fourth" {
            let mut index = 4;          
            let owned_string = String::from("On the ");           
            let together = owned_string + item + day_christmas_love;
            println!("{:?}", together);
          loop {             
               index -= 1;
             println!("{}", songs[index]);     
          if index <= 0 {
              break;
          };
        };
        }   else if item == "fifth" {
            let mut index = 5;          
            let owned_string = "On the ".to_owned();           
            let together = owned_string + item + day_christmas_love;
            println!("{:?}", together);
           loop {             
               index -= 1;
             println!("{}", songs[index]);     
          if index <= 0 {
              break;
          };
        };
    } else if item == "sixth" {
        let mut index = 6;          
        let owned_string = "On the ".to_owned();           
        let together = owned_string + item + day_christmas_love;
        println!("{:?}", together);
       loop {             
           index -= 1;
         println!("{}", songs[index]);     
      if index <= 0 {
          break;
      };
    };
} else if item == "seventh" {
    let mut index = 7;          
    let owned_string = "On the ".to_owned();           
    let together = owned_string + item + day_christmas_love;
    println!("{:?}", together);
   loop {             
       index -= 1;
     println!("{}", songs[index]);     
  if index <= 0 {
      break;
  };
};
} else if item == "eighth" {
    let mut index = 8;          
    let owned_string = "On the ".to_owned();           
    let together = owned_string + item + day_christmas_love;
    println!("{:?}", together);
     loop {             
       index -= 1;
     println!("{}", songs[index]);     
  if index <= 0 {
      break;
  };
};
} else if item == "ninth" {
    let mut index = 9;          
    let owned_string = "On the ".to_owned();           
    let together = owned_string + item + day_christmas_love;
    println!("{:?}", together);
  loop {             
       index -= 1;
     println!("{}", songs[index]);     
  if index <= 0 {
      break;
  };
};
} else if item == "tenth" {
    let mut index = 10;          
    let owned_string = "On the ".to_owned();           
    let together = owned_string + item + day_christmas_love;
    println!("{:?}", together);
  loop {             
       index -= 1;
     println!("{}", songs[index]);     
  if index <= 0 {
      break;
  };
};
} else if item == "eleventh" {
    let mut index = 11;          
    let owned_string = "On the ".to_owned();           
    let together = owned_string + item + day_christmas_love;
    println!("{:?}", together);
     loop {             
       index -= 1;
     println!("{}", songs[index]);     
  if index <= 0 {
      break;
  };
};
} else if item == "twelfth" {
    let mut index = 12;          
         let owned_string = String::from("On the ");                     
    let together = owned_string + item + day_christmas_love;
    println!("{:?}", together);
  loop {             
       index -= 1;
     println!("{}", songs[index]);     
  if index <= 0 {
      break;
  };
};
}
        else {
            println!("yes {}", item);
        };
    };

}
fn main() {
    twelve_days_of_christmas();
//     let  array: [i32; 7] =  [1,2,3,4,5,7,6];
//     let result = fibonacci_func( array, 2);
//    println!("array {:?}", result)
//   let rest =  convert_celsius_to_fahrenheit(26);
//   let restc = convert_fahrenheit_to_celsius(80);
//   println!("26 celsius to fahrengheit is {}", rest);
//   println!("78 fahrenheit is {}", restc);

}
