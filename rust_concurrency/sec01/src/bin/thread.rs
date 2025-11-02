use std::thread;

fn main() {
    let th1 = thread::spawn(f);
    let th2 = thread::spawn(f);

    println!("Hello from the main thread.");
    println!("Main thread id: {:?}", thread::current().id());
    
    let numbers = vec![1, 2, 3];
    
    let th3 = thread::spawn(move || {
        for n in numbers {
            println!("th3: {n}");
        }
    });
    
    let numbers = Vec::from_iter(0..=10000);
    
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    static X: [i32; 3] = [1, 2, 3];

    thread::spawn(|| dbg!(&X)).join().unwrap();
    thread::spawn(|| dbg!(&X)).join().unwrap();

    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    thread::spawn(move || dbg!(x)).join().unwrap();
    thread::spawn(move || dbg!(x)).join().unwrap();
    
    th1.join().unwrap();
    th2.join().unwrap();

    let average = t.join().unwrap();
    println!("average: {average}");

}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}