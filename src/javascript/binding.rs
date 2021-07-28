use rusty_v8 as v8;

/// `create_context_with` takes a HandleScope to `v8::Isolate` object
/// and returns a new HandleScope to newly created `v8::Context`.
pub fn create_context_with<'s>(
    isolate_scope: &mut v8::HandleScope<'s, ()>,
) -> v8::Local<'s, v8::Context> {
    let scope = &mut v8::EscapableHandleScope::new(isolate_scope);

    // create context
    let context = v8::Context::new(scope);

    // return with a handle to newly created v8::Context
    scope.escape(context)
}
