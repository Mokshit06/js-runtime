use std::cell::RefCell;
use std::rc::Rc;
use v8;

pub struct IsolateState {
    pub context: Option<v8::Global<v8::Context>>,
    pub module_map: crate::module::ModuleMap,
}

impl IsolateState {
    pub fn new(context: v8::Global<v8::Context>) -> Rc<RefCell<IsolateState>> {
        Rc::new(RefCell::new(IsolateState {
            context: Some(context),
            module_map: crate::module::ModuleMap::new(),
        }))
    }

    pub fn get(scope: &mut v8::Isolate) -> Rc<RefCell<Self>> {
        scope
            .get_slot::<Rc<RefCell<IsolateState>>>()
            .unwrap()
            .clone()
    }

    pub fn context(&self) -> v8::Global<v8::Context> {
        match &self.context {
            Some(c) => c.clone(),
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
