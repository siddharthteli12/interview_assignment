use std::collections::HashMap;

mod external {
    /// performs a call to a third party API
    pub async fn api_call(query_param: &str) -> String {
      unimplemented!();
    }
    use core::marker::PhantomData;
    use core::fmt::Debug;
    pub struct Stream<T: Send + 'static + Debug> {
      _pd: PhantomData<T>,
    }
    impl<T: Send + 'static + Debug> Stream<T> {
      /// returns the next element of the stream
      pub async fn next(&self) -> Option<T> {
        unimplemented!();
      }
    }
  }
  type Query = String;
  pub async fn do_stuff_here(stream: external::Stream<Query>) {
      let mut query_map: HashMap<String, String> = HashMap::new();
    while let Some(query) =  stream.next().await {
        if !query_map.contains_key(&query) {
            // Process
            let response = external::api_call(&query).await;
            println!("Response from stream - {:?}", response);
            // Push to vec
            query_map.insert(query.to_string(), response);
        }
        else {
            println!("Response from stream - {:?}", query_map.get(&query));
        }
    }
  }


  fn main() {

  }