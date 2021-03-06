use std::collections::HashMap;

// https://users.rust-lang.org/t/function-pointers-in-a-hashmap/6530/2
type Callback = fn(String) -> Option<()>;

struct EventHandler {
    user_function: HashMap<String, Callback>,
}

impl EventHandler {
    fn add_user_function(&mut self, name: String, func: Callback) {
        self.user_function.insert(name, func);
    }
}

fn script_foo(_arg: String) -> Option<()> {
    println!(".. execute foo");
    Some(())
}

impl EventHandler {
    fn on_script_call(&mut self, name: &str, argv: &[String]) -> Option<()> {
        let args = argv.iter().map(|ref x| format!("{}", &x)).collect::<Vec<String>>().join(", ");
        self.user_function[name](args);
        None
    }
}

#[test]
fn fn_map_test() {
    let mut handler = EventHandler { user_function: HashMap::new() };
    handler.add_user_function("CallFoo".to_string(), script_foo);
    handler.on_script_call("CallFoo", &["".to_string()]);
}
