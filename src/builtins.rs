use std::convert::TryFrom;
use std::fs;

use v8;

use crate::validator;

fn logger(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    println!("{}", args.get(0).to_rust_string_lossy(scope))
}

fn read_file_sync(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if let Some(path) = validator::validate_string(scope, &args.get(0), "path") {
        let contents = fs::read_to_string(path).unwrap();

        rv.set(v8::String::new(scope, contents.as_str()).unwrap().into())
    }
}

fn read_dir_sync(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if let Some(path) = validator::validate_string(scope, &args.get(0), "path") {
        let files = fs::read_dir(path)
            .unwrap()
            .map(|path| -> v8::Local<v8::Value> {
                v8::String::new(scope, path.unwrap().file_name().to_str().unwrap())
                    .unwrap()
                    .into()
            })
            .collect::<Vec<_>>();

        rv.set(v8::Array::new_with_elements(scope, &files[..]).into())
    }
}

fn cwd(scope: &mut v8::HandleScope, _args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    rv.set(
        v8::String::new(scope, std::env::current_dir().unwrap().to_str().unwrap())
            .unwrap()
            .into(),
    )
}

// async fn make_dir<'a>(
//     scope: &mut v8::HandleScope<'a>,
//     args: v8::FunctionCallbackArguments<'a>,
//     _rv: v8::ReturnValue<'a>,
// ) {
//     tokio::fs::create_dir(args.get(0).to_rust_string_lossy(scope))
//         .await
//         .unwrap();
// }

fn make_dir_sync(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if let Some(path) = validator::validate_string(scope, &args.get(0), "path") {
        fs::create_dir(path).unwrap();
    }
}

fn write_text_file_sync(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if let Some(path) = validator::validate_string(scope, &args.get(0), "path") {
        if let Some(contents) = validator::validate_string(scope, &args.get(1), "contents") {
            println!("path: {}, contents: {}", path, contents);
            fs::write(path, contents).unwrap()
        }
    }
}

pub struct Builtins {}

impl Builtins {
    pub fn create(scope: &mut v8::HandleScope) {
        let bindings = v8::Object::new(scope); // {}

        macro_rules! binding {
            ($name: expr, $fn: ident) => {
                let name = v8::String::new(scope, $name).unwrap();
                let value = v8::Function::new(scope, $fn).unwrap();

                bindings.set(scope, name.into(), value.into());
            };
        }

        binding!("printer", logger); // { printer: logger }
        binding!("readFileSync", read_file_sync); // { printer: logger, readFileSync: read_file_sync }
        binding!("makeDirSync", make_dir_sync);
        binding!("readDirSync", read_dir_sync);
        binding!("writeTextFileSync", write_text_file_sync);
        binding!("cwd", cwd);

        macro_rules! builtin {
            ($name: expr, $source: expr) => {{
                let val = match crate::script::run(scope, $source, $name) {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                };

                let func = v8::Local::<v8::Function>::try_from(val).unwrap();
                let recv = v8::undefined(scope).into();
                let args = [bindings.into()];
                func.call(scope, recv, &args).unwrap();
            }};
        }

        builtin!(
            "console.js",
            r#"({ printer }) => {
                ['log', 'error', 'warn'].map(level => {
                    Object.defineProperty(globalThis.console, level, {
                        value: printer
                    })
                })
            }"#
        );

        builtin!(
            "fs.js",
            r#"(runtime) => {
                globalThis.Runtime = runtime;
            }"#
        )
    }
}
