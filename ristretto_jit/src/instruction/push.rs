use cranelift::codegen::ir::Value;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.bipush>
pub(crate) fn bipush(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>, value: i8) {
    let value = i64::from(value);
    let value = function_builder.ins().iconst(types::I32, value);
    stack.push(value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.sipush>
pub(crate) fn sipush(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>, value: i16) {
    let value = i64::from(value);
    let value = function_builder.ins().iconst(types::I32, value);
    stack.push(value);
}
