use std::{fs::File, io::Write};

use graphviz_rust::{cmd::CommandArg, dot_structures::*, printer::PrinterContext};
use minimax::{
    game::*,
    minimax::{minimax, DecisionTreeNode, MinimaxDriver},
};

fn main() -> std::io::Result<()> {
    // graph parameters
    const MAX_DEPTH: i32 = 10;
    const ALTERNATIVES_TO_DRAW: usize = 2;
    const MINIMAX_DEPTH: Option<u32> = None;
    let state = "
    ...
    OXX
    ..O";
    let game = minimax::tictactoe::TicTacToeGame::from_state(state, Player::X);

    // get the decision tree
    let decision_tree = minimax(&game, MINIMAX_DEPTH);

    // build the graph
    let mut graph = make_graph();
    graph_tree(&mut graph, decision_tree, Box::new(game), MAX_DEPTH, ALTERNATIVES_TO_DRAW);

    // print it to string
    let mut printer_context = PrinterContext::default();
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
    alternatives_to_draw: usize,
) {
    graph_node(
        graph,
        decision_tree,
        game,
        0,
        max_depth,
        &mut 0,
        alternatives_to_draw,
    );
}

fn graph_node(
    graph: &mut Graph,
    decision_tree: minimax::minimax::DecisionTreeNode,
    game: Box<dyn MinimaxDriver>,
    depth: i32,
    max_depth: i32,
    node_id: &mut i32,
    alternatives_to_draw: usize,
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

    if decision_tree.moves.len() == 0 {
        return Some(current_node)
    }

    // prepare moves to iterate over
    let score_factor = game.get_current_player().score_multiplier();
    let mut all_moves: Vec<(Move, DecisionTreeNode)> = decision_tree.moves.into_iter().collect();
    all_moves.sort_by_key(|(_, n)| -n.score * score_factor);

    // always get best move in the front of the list
    // if there are multiple moves with the same highest score, the selected one might not be plotted
    // not the most efficient way to do it, but this is not a hotspot
    let best_move_idx = all_moves.iter().position(|m| m.0 == decision_tree.best_move.unwrap()).unwrap();
    if best_move_idx != 0 {
        all_moves.rotate_right(1);
        let new_best_idx = (best_move_idx + 1) % all_moves.len();
        all_moves.swap(0, new_best_idx);
    }

    // draw nodes ordered by score
    for (m, tree_node) in all_moves.into_iter().take(1 + alternatives_to_draw) {
        let new_game = game.apply_move(m);
        let child_node = graph_node(
            graph,
            tree_node,
            new_game,
            depth + 1,
            max_depth,
            node_id,
            alternatives_to_draw,
        );
        let (color_edge, is_heavy) = if decision_tree.best_move.unwrap() == m {
            (get_player_color(game.get_current_player()), true)
        } else {
            ("black", false)
        };
        if let Some(child) = child_node {
            add_edge(
                graph,
                current_node.clone(),
                child,
                color_edge.to_string(),
                is_heavy,
            );
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

fn add_edge(graph: &mut Graph, node1: NodeId, node2: NodeId, color: String, is_heavy: bool) {
    let mut attrs = vec![Attribute(
        Id::Plain("color".into()),
        Id::Plain(color.into()),
    )];
    if is_heavy {
        attrs.push(Attribute(
            Id::Plain("weight".into()),
            Id::Plain("10".into()),
        ));
        attrs.push(Attribute(
            Id::Plain("penwidth".into()),
            Id::Plain("2".into()),
        ));
    }
    graph.add_stmt(Stmt::Edge(Edge {
        ty: EdgeTy::Pair(Vertex::N(node1), Vertex::N(node2)),
        attributes: attrs,
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
