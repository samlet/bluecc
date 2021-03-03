fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 闭包由特征表示，这意味着您不能直接返回闭包。在大多数情况下，您可能希望返回特征，
// 而可以使用将特征实现为函数的返回值的具体类型。但是您不能使用闭包来做到这一点，
// 因为它们没有可返回的具体类型。例如，您不允许将函数指针fn用作返回类型。
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
#[test]
fn fun_test() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

#[test]
fn closure_works() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    //  list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);
}

// 可以利用元组结构和元组结构枚举变量的实现细节。这些类型()用作初始化器语法，看起来像一个函数调用。
// 初始化程序实际上是作为返回由其参数构造的实例的函数实现的。我们可以将这些初始化函数用作实现闭包特征的函数指针，
// 这意味着我们可以将初始化函数指定为采用闭包的方法的参数
#[test]
fn enum_works() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}",list_of_statuses);
}