use std::collections::HashSet;
use std::fmt::Write;
use std::io;
use std::path::Path;

use graphviz_rust::{
    cmd::{CommandArg, Format},
    exec_dot,
};

use crate::value::ValueRef;

/// Builds a Graphviz DOT computation graph rooted at `value`.
///
/// Each scalar is a record-shaped node. Every operation receives a separate
/// node so the graph reads from operands to operation to result, as in the
/// micrograd visualizations.
pub fn to_dot(value: &ValueRef) -> String {
    let mut dot = String::from("digraph computation_graph {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=record];\n");

    let mut visited = HashSet::new();
    add_value(&mut dot, value, &mut visited);

    dot.push_str("}\n");
    dot
}

/// Renders the computation graph as an SVG using the installed `dot` command.
pub fn render_svg(value: &ValueRef, output: impl AsRef<Path>) -> io::Result<()> {
    let svg = exec_dot(to_dot(value), vec![CommandArg::Format(Format::Svg)])?;
    std::fs::write(output, svg)
}

fn add_value(dot: &mut String, value: &ValueRef, visited: &mut HashSet<usize>) {
    let (id, data, grad, operation, parents) = value.graph_snapshot();
    if !visited.insert(id) {
        return;
    }

    let current_value_id = value_id(id);
    writeln!(
        dot,
        "  {current_value_id} [label=\"{{ data {data} | grad {grad} }}\"];"
    )
    .expect("writing to String");

    if let Some(operation) = operation {
        let operation_id = operation_id(id);
        writeln!(
            dot,
            "  {operation_id} [label=\"{operation}\", shape=circle];"
        )
        .expect("writing to String");
        writeln!(dot, "  {operation_id} -> {current_value_id};").expect("writing to String");

        for parent in &parents {
            add_value(dot, parent, visited);
            let (parent_id, ..) = parent.graph_snapshot();
            writeln!(dot, "  {} -> {operation_id};", value_id(parent_id))
                .expect("writing to String");
        }
    }
}

fn value_id(id: usize) -> String {
    format!("value_{id}")
}

fn operation_id(id: usize) -> String {
    format!("operation_{id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_graph_contains_value_and_operation_nodes() {
        let a = ValueRef::leaf(2);
        let b = ValueRef::leaf(3);
        let result = a.clone() * b.clone();
        let dot = to_dot(&result);

        assert!(dot.contains("rankdir=LR"));
        assert_eq!(dot.matches("label=\"{ data 2 | grad 0 }\"").count(), 1);
        assert_eq!(dot.matches("label=\"{ data 3 | grad 0 }\"").count(), 1);
        assert_eq!(dot.matches("label=\"{ data 6 | grad 0 }\"").count(), 1);
        assert_eq!(dot.matches("label=\"*\"").count(), 1);
        assert_eq!(dot.matches("shape=circle").count(), 1);
    }
}
