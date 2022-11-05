use std::collections::HashMap;

// HashMap(辞書型)は異なる型を値として持てなかったため、こういったややこしい実装になっている。
// 参考：https://zenn.dev/j5ik2o/articles/21d477b8dbbf70

//#[derive(Debug)]
struct MyHashMap {
    values: HashMap<String, Box<dyn core::any::Any>>
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    fn set<T: core::any::Any>(&mut self, key: String, value: T) {
        self.values.insert(key, Box::new(value));
    }

    fn get<T: core::any::Any>(&self, key: &String) -> Option<&T> {
        self.values.get(key).and_then(|v| v.downcast_ref::<T>())
    }
}


fn main() {
    let mut tx = MyHashMap::new();
    tx.set("a".to_string(), 1243);
    tx.set("b".to_string(), "asd");

    println!("{:?}", tx.get::<i32>(&"a".to_string()).unwrap());
    println!("{:?}", tx.get::<&str>(&"b".to_string()).unwrap());
    
}