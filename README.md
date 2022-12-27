# quick_js

quick-js는 Fabrice Bellard의 새로운 Javascript 엔진인 QuickJS용 Rust 래퍼이다. Rust에서 최신 Javascript를 쉽고 간단하게 실행할 수 있다. 

즉,  rust에서 javascript를 실행하려고 할 때 사용한다. 

> cargo run 할 때마다 다시 빌드하는 것 같고 실행이 느리다. 윈도우즈에서는 아직 지원하지 않는다.  외부 자바스크립트 파일을 임포트할 수 있는지 분명하지 않다. 


공식 문서인 [quick_js](https://docs.rs/quick-js/latest/quick_js/#enums)를 참고한다. 

**개발환경**     
* Linux latte 5.15.79.1-microsoft-standard-WSL2 #1 SMP Wed Nov 23 01:01:46 UTC 2022 x86_64 x86_64 x86_64 GNU/Linux
* node v19.0.1 
* cargo 1.66.0 (d65d197ad 2022-11-15)



**패키지 생성**    
```shell
cargo new quickjs-test 
```

**Cargo.toml**    
```toml
[package]
name = "quickjs-test"
version = "0.1.0"
edition = "2021"

[dependencies]
quick-js = "0.2.2"
```

**main.rs**   
```rust
use quick_js::{Context, JsValue};

fn main() {
    let context = Context::new().unwrap();

    // Eval.

    let value = context.eval("1 + 2").unwrap();
    //assert_eq!(value, JsValue::Int(3));
    println!("value: {:?}", &value);
}
```
다음을 실행한다. 
```shell
cargo run
```
다음의 결과가 출력된다. 
```shell
warning: `quickjs-test` (bin "quickjs-test") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 12.18s
     Running `target/debug/quickjs-test`
value: Int(3)
```






