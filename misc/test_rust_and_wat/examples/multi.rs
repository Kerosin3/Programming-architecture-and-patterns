use anyhow::Result;
//https://github.com/bytecodealliance/wasmtime/blob/main/examples/multi.rs
fn main() -> Result<()> {
    use wasmtime::*;
    println!("initializing multi example");
    //init engine
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    // compile module
    let module = Module::from_file(&engine, "examples/multi.wat")?;
    //--------------------
    println!("creating callback function");
    let callback_f = Func::wrap(&mut store, |a: i32, b: i64| -> (i64, i32) {
        (b * 5, a * 5)
    });
    println!("Instantiating the module");
    let instance = Instance::new(&mut store, &module, &[callback_f.into()])?;

    println!("extracting exports..");
    let g = instance.get_typed_func::<(i32, i64), (i64, i32)>(&mut store, "g")?;
    let (a, b) = g.call(&mut store, (2, 5))?;
    println!("result is:");
    println!("> {} {}", a, b);
    Ok(())
}
