
fn intro() {
     struct A;
     struct SingletonGen<T>(T);

     let _a = SingletonGen(42);
     let _b = SingletonGen("asd");
     let _c = SingletonGen(A);
}

fn functions() {
     struct A;
     struct SingletonGen<T>(T);

     fn gen_specialized_i32(_t: SingletonGen<i32>) {

     }

     fn gen_specialized_a(_t: SingletonGen<A>) {

     }

     fn gen<T>(_t: SingletonGen<T>) {

     }

     let _a = SingletonGen(42);
     gen_specialized_i32(_a);

     let _b = SingletonGen(A);
     gen_specialized_a(_b);

     let _c = SingletonGen(42);
     let _d = SingletonGen(A);
     gen(_c);
     gen(_d);
}

fn impls() {
     struct GenVal<T>(T);
     impl <T> GenVal<T> {
          fn value(&self) -> &T {
               &self.0
          }
     }

     let v = GenVal(32);
     assert_eq!(v.value(), &32);
}

pub fn main() {
     intro();
     functions();
     impls();
}