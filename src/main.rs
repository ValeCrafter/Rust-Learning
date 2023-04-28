use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        p { "Hello, World!" }
        div (class="test-class"){
            p{"Hello, World, but red!"}
        }
        button(on:click=|_| {
            let mut count:i32 = 0;
            count += 1;
        }) {
            ("Click me!")
        }
    });
}
