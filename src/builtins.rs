use std::convert::TryFrom;

use v8;

fn logger(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    println!("{:#?}", args)
}

pub struct Builtins {}

impl Builtins {
    pub fn create(scope: &mut v8::HandleScope) {
        let bindings = v8::Object::new(scope);

        let name = v8::String::new(scope, "printer").unwrap();
        let value = v8::Function::new(scope, logger).unwrap();

        bindings.set(scope, name.into(), value.into());

        let source = "({printer}) => { globalThis.p = printer }";
        let val = match crate::script::run(scope, source, "console.js") {
            Ok(v) => v,
            Err(_) => unreachable!(),
        };

        let func = v8::Local::<v8::Function>::try_from(val).unwrap();
        let recv = v8::undefined(scope).into();
        let args = [bindings.into()];
        func.call(scope, recv, &args).unwrap();
    }
}
