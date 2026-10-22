use micrograd::{graph, value::ValueRef};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = ValueRef::leaf(9);
    let b = ValueRef::leaf(10);
    let result = a + b;

    print!("{}", graph::to_dot(&result));
    graph::render_svg(&result, "computation.svg")?;

    Ok(())
}
