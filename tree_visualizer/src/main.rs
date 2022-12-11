use std::{default, fs::File, io::Write};

use graphviz_rust::{
    cmd::CommandArg,
    dot_structures::*,
    printer::{DotPrinter, PrinterContext},
};
use minimax::{game::*, minimax::minimax, tictactoe::*};

fn main() -> std::io::Result<()> {
    // get the decision tree
    let state = "
        X.X
        X.O
        O.O";
    let game = TicTacToeGame::from_state(state, Player::O);
    // let decision_tree = minimax(&game);

    // create graph
    let mut graph = Graph::DiGraph {
        id: Id::Plain("decision_tree".into()),
        strict: true,
        stmts: Default::default(),
    };
    let node1 = add_node(&mut graph, "example_node");
    let node2 = add_node(&mut graph, "example_node2");
    let node3 = add_node(&mut graph, "example_node3");
    add_edge(&mut graph, node1.clone(), node2);
    add_edge(&mut graph, node1, node3);

    // print it
    let mut printer_context = PrinterContext::default();
    println!("{}", graph.print(&mut printer_context));
    let graph_svg = graphviz_rust::exec(
        graph,
        &mut printer_context,
        vec![CommandArg::Format(graphviz_rust::cmd::Format::Svg)],
    )
    .unwrap();

    let mut output_file = File::create("target/graph.svg")?;

    write!(output_file, "{}", graph_svg)
}

fn add_edge(graph: &mut Graph, node1: NodeId, node2: NodeId) {
    graph.add_stmt(Stmt::Edge(Edge {
        ty: EdgeTy::Pair(Vertex::N(node1), Vertex::N(node2)),
        attributes: vec![],
    }));
}

fn add_node(graph: &mut Graph, arg: &str) -> NodeId {
    let id = NodeId(Id::Plain(arg.into()), None);
    let returned_id = id.clone();
    let node = Node::new(id, vec![]);
    graph.add_stmt(Stmt::Node(node));
    returned_id
}
