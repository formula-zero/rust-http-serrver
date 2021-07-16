use std::{collections::HashMap, hash::Hash};
#[derive(Debug)]
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>), // heap allocated array
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {    // Why not TryFrom ?!
  fn from(s: &'buf str) -> Self {
    let mut data = HashMap::new();
    for sub_str in s.split('&') {
      let mut key = sub_str;
      let mut val = "";

      if let Some(i) = sub_str.find('=') {    //if let - find returns 'Option'
        key = &sub_str[..i];
        val = &sub_str[i+1 ..];
      }

      data.entry(key)
      .and_modify(|existing| match existing {
        Value::Single(prev_val) => {
          *existing = Value::Multiple(vec![prev_val, val]);
          /*
          let mut vec = vec![prev_val, val];  // macro !
          existing = &mut Value::Multiple(vec);
          */
          /*
          let mut vec = Vec::new();
          vec.push(val);
          vec.push(prev_val);
          */
        }
        Value::Multiple(vec) => vec.push(val)
      })
      .or_insert(Value::Single(val));
    }


    QueryString { data }


    //unimplemented!()
  }
}