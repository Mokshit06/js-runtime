use v8;

pub fn validate_string(
    scope: &mut v8::HandleScope,
    arg: &v8::Local<v8::Value>,
    arg_name: &str,
) -> Option<String> {
    if arg.is_string() {
        Some(arg.to_rust_string_lossy(scope))
    } else {
        let message = v8::String::new(
            scope,
            format!(r#"The "{}" argument must be of type string."#, arg_name).as_str(),
        )
        .unwrap();
        let exception = v8::Exception::type_error(scope, message);

        scope.throw_exception(exception);

        None
    }
}
