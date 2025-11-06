struct Foo {
    val: u32,
}

fn main() {
    let mut x = Foo{val: 10};
    {
        let a = &mut x;
        println!("a.val = {}", a.val);
    
        // println!("x.val = {}", x.val);

        let b: &Foo = a;
        println!("b.val = {}", b.val);

        a.val = 30;
    }

    {
        let c = &x;
        println!("c.val = {}", c.val);
        println!("x.val = {}", x.val);

        println!("c.val = {}", c.val);
    }

    println!("x.val = {}", x.val);
}