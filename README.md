Ref

## Is a simple `Arc<Mutex<T>` system that allow you to read always the value, but act like Mutex on mutating data!

# :warning:Only use if you know what are you doing!

---

# Examples:

- 1:
  
  ```rust
  use ref_kman::Ref;
  
  pub struct State{
      pub data: i32
  }
  
  impl State{
      pub fn new() -> Self{
          Self{
              data: 0
          }
      }
  }
  
  fn main(){
      let data = Ref::new(State::new());
      // clone is acting like `Arc::clone()`
      // because RefInner is containt in a `Arc`
      // and you can share with others threads!
      let data_clone = data.clone();
      // is not mutabile you need to get the motabile RefMut<T>
      // i create a scope because is needed to not block the thread.
      {
      let mut mut_data = data.get_mut();
      // i can read the original data but no other thread can modifiy
      println!("Data: {}", data.data);
      // Data: 0
      mut_data.data = 5;
      }
      println!("Data: {}", data_clone.data);
      // Data: 5
      
      // and you can create a closure
      // this is better because you can call 2 times.
      // RefMut<T>
      // You only read and modify data
      // You cannot clone
      data.mut_scope(|mut_data|{
          mut_data.data = 10;
      });
      println!("Data: {}", data.data);
  }
  ```
  
  


