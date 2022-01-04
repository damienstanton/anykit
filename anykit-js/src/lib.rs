use neon::prelude::*;
use anykit::math;

fn sum_as_string(mut cx: FunctionContext) -> JsResult<JsString> {
    // NOTE: arguments are accessed from the context by position.
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    
    Ok(cx.string(math::add(a, b).to_string()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("sum_as_string", sum_as_string)?;
    Ok(())
}
