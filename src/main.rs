use js_runtime;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "jstime", rename_all = "kebab-case")]
struct Opt {
    #[structopt()]
    filename: String,
}

fn main() {
    let opt = Opt::from_args();

    js_runtime::init();

    let mut runtime = js_runtime::JSRuntime::new();

    std::process::exit(match runtime.import(&opt.filename) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    });
    // let isolate = &mut v8::Isolate::new(Default::default());

    // let scope = &mut v8::HandleScope::new(isolate);
    // let context = v8::Context::new(scope);
    // let scope = &mut v8::ContextScope::new(scope, context);

    // // let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
    // let code = v8::String::new(scope, "`Hello ${{a: 'b'}.a}`").unwrap();
    // println!("javascript code: {}", code.to_rust_string_lossy(scope));

    // let script = v8::Script::compile(scope, code, None).unwrap();
    // let result = script.run(scope).unwrap();
    // let result = result.to_string(scope).unwrap();
    // println!("result: {}", result.to_rust_string_lossy(scope));
}
