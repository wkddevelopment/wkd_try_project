use std::collections::HashMap;

fn main() {

   let mut myvector = vec![4, 1, 99, 1, 1,  2, 3, 4, 5, 4, 3, 3, 4, 3, 2, 3, 4, 2, -7, -3, 8, -101];
   myvector.sort();

   let mut numbsintext: String = String::from("");

   for i in myvector {
      // print sorted number of the vector
      println!("{}", &i); 

      // build string for hashmap
      let a = &i.to_string();
      numbsintext.push_str(&a);
      numbsintext.push_str(" ");
   }


   // Hashmap zum zählen wi oft die Zahl (String) vorkommt
   let mut myhashmap = HashMap::new();
   for number in numbsintext.split_whitespace(){
      let count = myhashmap.entry(number).or_insert(0);
      *count += 1;
   }

   println!("{:?}", myhashmap);

}
