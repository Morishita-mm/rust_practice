use std::sync::RwLock; // ❶

fn main() {
    let lock = RwLock::new(10); // ❷
    {
        // immutableな参照を取得 ❸
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }

    {
        // mutableな参照を取得 ❹
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
}