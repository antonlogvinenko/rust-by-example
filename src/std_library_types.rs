fn boxing() {
     use std::mem;

     struct Point {
          _x: f64,
          _y: f64,
     }

     impl Default for Point {
          fn default() -> Self {
               Point { _x: 0.0, _y: 0.0 }
          }
     }
     let point: Point = Default::default();
     let boxed_point: Box<Point> = Box::new(Default::default());
     let double_boxed_point: Box<Box<Point>> = Box::new(Box::new(Default::default()));

     assert_eq!(mem::size_of_val(&point), 16);
     assert_eq!(mem::size_of_val(&boxed_point), 8);
     assert_eq!(mem::size_of_val(&double_boxed_point), 8);
}

fn vectors() {
     //pointer to data, length, capacity
     //when size reaches capacity, vector gets reallocated with larger capacity

     //create vector
     let _collected_vector: Vec<i32> = (0..10).collect();
     let mut xs = vec![1, 2, 3];

     //modify
     assert_eq!(xs.len(), 3);
     xs.push(1);
     assert_eq!(xs.len(), 4);
     assert_eq!(xs.pop(), Some(1));
     let mut sum = 0;
     for x in xs {
          sum += x;
     }
     assert_eq!(sum, 6);
}

fn strings() {
     //string - vec
     let mut str = String::new();
     
     str.push('a');
     assert_eq!(str, "a");
     str.push_str(" cake");
     assert_eq!(str, "a cake");

     let new_str = str.replace("cake", "baked cake");
     assert_eq!(new_str, "a baked cake");

     //string literal - slice
     let literal: &str = "some literal string";
     let split = literal.split_whitespace().skip(2).take(1).next();
     assert_eq!(split, Some("string"));
     let chars: Vec<char> = literal.chars().collect();
     assert_eq!(chars.len(), 19);

     //bytestring literal - array
     let bytestring: &[u8; 21] = b"this is a byte string";
     assert_eq!(bytestring.len(), 21);

     //literals features
     //1. escaping
     let lit1 = "I'm writing \x52\x75\x73\x74!";
     assert_eq!(lit1, "I'm writing Rust!");
     //utf code points
     let lit2 = "\u{211D}";
     assert_eq!(lit2, "ℝ");
     //multiple lines + escaped multiple lines
     let lit3 = "Cake
                    is a \
                    lie";
     assert_eq!(lit3, "Cake\n                    is a lie");
     let lit4 = r"No escapes\n but none needed: ℝ";
     assert_eq!(lit4, "No escapes\\n but none needed: ℝ");
     let lit5 = r#"Let's use these: "" "#;
     assert_eq!(lit5, "Let's use these: \"\" ");
     let lit6 = r##"Let's use these: ## "##;
     assert_eq!(lit6, "Let's use these: ## ");

     //literal features are applicable to byte strings (br or br# for raw)
}

pub fn main() {
     boxing();
     vectors();
     strings();
}
