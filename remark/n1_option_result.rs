struct Person {
    name: Option<String>,
}

#[allow(unused)]
fn my_test() {
    let x = Some(Person { name: Some("张三".to_string()) });
    let y = x.map(|s| s.name);
    // y 是 Option<Option<String>>, s.name 是 Option<String>,map又包裹一次
    println!("{:?}", y);

    // and_then要求返回值是 Option<T> 或 Result<T, E>，自己不会自动包裹，需要自己包裹
    let x = Some(Person { name: Some("张三".to_string()) });
    let z = x
        .and_then(|s| s.name)
        .ok_or_else(|| anyhow::anyhow!("none"))
        .and_then(|s| Ok(format!("{} 的年龄是18岁", s)));

    println!("{:?}", z);

    let x = format!("{} 的年龄是18岁", "张三");

    let m = Some(12);
    let n = m.map(|s| s + 1); // map 自动包装成 Some()
    println!("{:?}", n);

    let m = Some(12);
    let n = m.and_then(|x| Some(x + 1)); // and_then 需要自己包装成 Some()
    let n1: Result<i32, String> = Result::Ok(1);
    let n2 = n1.ok(); // 将 Result 转换为 Option
    println!("{:?}", n2);
}
