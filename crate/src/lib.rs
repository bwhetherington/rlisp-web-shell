#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate lazy_static;

extern crate rlisp;
extern crate wasm_bindgen;
extern crate web_sys;
use rlisp::prelude::*;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn test_str(s: String) -> String {
    format!("String: {}", s)
}

use rlisp::{
    rlisp_interpreter::{
        context::Context,
        expression::{Callable, Expression},
    },
    rlisp_intrinsics::init_context,
    rlisp_parser::Parser,
};
use std::rc::Rc;

static mut CONTEXT: Option<Context> = None;

#[wasm_bindgen]
pub fn initialize() {
    set_panic_hook();
    unsafe {
        let mut ctx = init_context("1.0.0");
        let f = |_: &[Expression], _: &mut Context| Expression::default();
        let expr = Expression::Callable(Callable::Intrinsic(Rc::new(f)));
        ctx.insert("import", expr);

        CONTEXT = Some(ctx);
    }
}

#[wasm_bindgen]
pub fn set_entry_point(input: &str) {
    unsafe {
        if let Some(ref mut ctx) = CONTEXT {
            ctx.insert("__FILE__", input);
        }
    }
}

#[wasm_bindgen]
pub fn handle_input(input: &str) -> Option<String> {
    if input.len() > 0 {
        let mut parser = Parser::new(input.chars());
        let expr = parser.parse_expr()?;

        unsafe {
            if let Some(ref mut ctx) = CONTEXT {
                let res = expr.eval(ctx);
                if res.is_exception() {
                    Some(format!("exception:{}", res))
                } else {
                    Some(format!("value:{}", res))
                }
            } else {
                None
            }
        }
    } else {
        None
    }
}
