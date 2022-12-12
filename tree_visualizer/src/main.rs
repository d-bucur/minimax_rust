use std::{fs::File, io::Write};

use graphviz_rust::{cmd::CommandArg, dot_structures::*, printer::PrinterContext};
use minimax::{
    game::*,
    minimax::{minimax, MinimaxDriver},
};

fn main() -> std::io::Result<()> {
    // get the decision tree
    let state = "
        ...
        ...
        ...";
    let game = minimax::tictactoe::TicTacToeGame::from_state(state, Player::X);
    let decision_tree = minimax(&game, None);

    // build the graph
    let mut graph = make_graph();
    graph_tree(&mut graph, decision_tree, Box::new(game), 2, true);

    // print it
    let mut printer_context = PrinterContext::default();
    // println!("{}", graph.print(&mut printer_context));
    let graph_svg = graphviz_rust::exec(
        graph,
        &mut printer_context,
        vec![CommandArg::Format(graphviz_rust::cmd::Format::Svg)],
    )
    .unwrap();

    // save to file
    let mut output_file = File::create("target/graph.svg")?;
    write!(output_file, "{}", graph_svg)
}

fn graph_tree(
    graph: &mut Graph,
    decision_tree: minimax::minimax::DecisionTreeNode,
    game: Box<dyn MinimaxDriver>,
    max_depth: i32,
    empty_score_visible: bool,
) {
    graph_node(graph, decision_tree, game, 0, max_depth, &mut 0, empty_score_visible);
}

fn graph_node(
    graph: &mut Graph,
    decision_tree: minimax::minimax::DecisionTreeNode,
    game: Box<dyn MinimaxDriver>,
    depth: i32,
    max_depth: i32,
    node_id: &mut i32,
    empty_score_visible: bool,
    // TODO visualization parameters: terminal states always visible, selected move always visible
) -> Option<NodeId> {
    if depth > max_depth {
        return None;
    }
    let color_node = get_player_color(game.get_winner());
    let current_node = add_node(
        graph,
        format!("node_{}_{}", depth, node_id),
        format!("s: {}\n{:?}", decision_tree.score, game),
        color_node.into(),
    );

    let best_move = decision_tree.best_move;
    let move_iterator = decision_tree.moves.into_iter().filter(|(m, tree_node)| empty_score_visible || tree_node.score !=0 || *m == best_move.unwrap());
    for (m, tree_node) in move_iterator {
        let new_game = game.apply_move(m);
        let child_node = graph_node(
            graph,
            tree_node,
            new_game,
            depth + 1,
            max_depth,
            node_id,
            empty_score_visible,
        );
        let color_edge = if best_move.unwrap() == m {
            get_player_color(game.get_current_player())
        } else {
            "black"
        }
        .to_string();
        if let Some(child) = child_node {
            add_edge(graph, current_node.clone(), child, color_edge);
        }
        *node_id += 1;
    }
    Some(current_node)
}

fn get_player_color(player: Player) -> &'static str {
    match player {
        Player::X => "red",
        Player::O => "blue",
        Player::None => "black",
    }
}

fn make_graph() -> Graph {
    let font_attr = Attribute(
        Id::Plain("fontname".into()),
        Id::Plain("\"Courier,FreeMono\"".into()),
    );
    let width_attr = Attribute(Id::Plain("width".into()), Id::Plain("1.5".into()));
    Graph::DiGraph {
        id: Id::Plain("decision_tree".into()),
        strict: true,
        stmts: vec![
            Stmt::Attribute(font_attr.clone()),
            Stmt::GAttribute(GraphAttributes::Node(vec![font_attr, width_attr])),
        ],
    }
}

fn add_edge(graph: &mut Graph, node1: NodeId, node2: NodeId, color: String) {
    graph.add_stmt(Stmt::Edge(Edge {
        ty: EdgeTy::Pair(Vertex::N(node1), Vertex::N(node2)),
        attributes: vec![Attribute(
            Id::Plain("color".into()),
            Id::Plain(color.into()),
        )],
    }));
}

fn add_node(graph: &mut Graph, node_id: String, label: String, color: String) -> NodeId {
    let id = NodeId(Id::Plain(node_id), None);
    let returned_id = id.clone();
    let node = Node::new(
        id,
        vec![
            Attribute(
                Id::Plain("label".into()),
                Id::Escaped(format!("\"{}\"", label)),
            ),
            Attribute(Id::Plain("shape".into()), Id::Plain("box".into())),
            Attribute(Id::Plain("color".into()), Id::Plain(color.into())),
        ],
    );
    graph.add_stmt(Stmt::Node(node));
    returned_id
}
