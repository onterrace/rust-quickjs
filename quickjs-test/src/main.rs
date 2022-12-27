use quick_js::{Context, JsValue};

fn main() {
    let context = Context::new().unwrap();

    // Eval.

    let value = context.eval("1 + 2").unwrap();
    //assert_eq!(value, JsValue::Int(3));
    println!("value: {:?}", &value);
}