mod builtins;
mod isolate_state;
mod js_loading;
mod module;
mod script;
mod transformer;
mod validator;

use isolate_state::IsolateState;
use v8;

pub fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

pub struct JSRuntime {
    isolate: Option<v8::OwnedIsolate>,
}

impl JSRuntime {
    pub fn new() -> Self {
        let isolate = v8::Isolate::new(Default::default());

        JSRuntime::create(isolate)
    }

    pub fn create(mut isolate: v8::OwnedIsolate) -> Self {
        let global_context = {
            let scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);

            v8::Global::new(scope, context)
        };

        isolate.set_slot(IsolateState::new(global_context));

        {
            let context = IsolateState::get(&mut isolate).borrow().context();
            let scope = &mut v8::HandleScope::with_context(&mut isolate, context);
            builtins::Builtins::create(scope);
        }

        Self {
            isolate: Some(isolate),
        }
    }

    fn isolate(&mut self) -> &mut v8::Isolate {
        match self.isolate.as_mut() {
            Some(i) => i,
            None => unsafe {
                std::hint::unreachable_unchecked();
            },
        }
    }

    pub fn import(&mut self, filename: &str) -> Result<(), String> {
        let context = IsolateState::get(self.isolate()).borrow().context();
        let scope = &mut v8::HandleScope::with_context(self.isolate(), context);
        let loader = module::Loader::new();

        let mut cwd = std::env::current_dir().unwrap();
        cwd.push("js_runtime");
        let cwd = cwd.into_os_string().into_string().unwrap();

        match loader.import(scope, &cwd, filename) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string(scope).unwrap().to_rust_string_lossy(scope)),
        }
    }
}
