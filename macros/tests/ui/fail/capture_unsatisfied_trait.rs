#![feature(stmt_expr_attributes, proc_macro_hygiene)]

#[macro_use]
extern crate antlog_macros;

// Does not implement `Display`
struct Input;

fn main() {
    call(Input);
}

fn call(input: Input) {
    let kvs: &[(&str, antlog_macros_impl::__private::Value)] = &[
        #[display]__log_private_capture!(input),
    ];

    println!("{:?}", kvs);
}
