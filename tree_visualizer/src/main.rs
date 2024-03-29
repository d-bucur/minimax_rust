use std::{fs::File, io::Write, rc::Rc};

use graphviz_rust::{cmd::CommandArg, dot_structures::*, printer::PrinterContext};
use minimax::{game::*, minimax::*};
use tracing::info;

fn main() -> std::io::Result<()> {
    let collector = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();

    // tic tac toe setup
    // const MAX_DEPTH: i32 = 5;
    // const ALTERNATIVES_TO_DRAW: usize = 4;
    // let state = "
    // X..
    // .O.
    // ..X";
    // let game = minimax::tictactoe::TicTacToeGame::from_state(state, Player::O);
    // let mut minimax = Minimax::new(MinimaxParams::default());

    // connect 4 setup
    const MAX_DEPTH: i32 = 7;
    const ALTERNATIVES_TO_DRAW: usize = 0;
    let mut minimax = Minimax::new(MinimaxParams {
        max_depth: 7,
        ..Default::default()
    });
        let state = "
        .......
        .......
        ..X....
        X.O....
        O.X....
        XXOOOXO";
    let game = minimax::connect4::Connect4Game::from_state(state, None, Player::X);

    // get the decision tree
    let decision_tree = minimax.minimax(&game);

    // build the graph
    let mut graph = make_graph();
    graph_tree(
        &mut graph,
        decision_tree,
        Box::new(game),
        MAX_DEPTH,
        ALTERNATIVES_TO_DRAW,
    );

    match minimax.get_internal_stats() {
        (total, last, cache) => {
            info!(
                "Stats: nodes_examined_total={}, nodes_examined_last_run={}, cache_size={}",
                total, last, cache
            );
        }
    }

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
    decision_tree: Rc<minimax::minimax::DecisionTreeNode>,
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
        4,  // pretty hacky, doesn't work if pruned
    );
}

fn graph_node(
    graph: &mut Graph,
    decision_tree: Rc<minimax::minimax::DecisionTreeNode>,
    game: Box<dyn MinimaxDriver>,
    depth: i32,
    max_depth: i32,
    node_id: &mut i32,
    alternatives_to_draw: usize,
    draw_all_at_depth: i32
) -> Option<NodeId> {
    if depth > max_depth {
        return None;
    }
    let color_node = get_player_color(game.get_winner());
    let current_node = add_node(
        graph,
        format!("node_{}_{}", depth, node_id),
        format!(
            "sc: {} de: {}\n{:?}\na: {} b: {}",
            decision_tree.score, depth, game, decision_tree.alfa, decision_tree.beta
        ),
        color_node.into(),
    );

    if decision_tree.moves.len() == 0 {
        return Some(current_node);
    }

    // prepare moves to iterate over
    let score_factor = game.get_current_player().score_multiplier();
    let mut all_moves: Vec<(Move, Rc<DecisionTreeNode>)> = decision_tree
        .moves
        .iter()
        .map(|(p, t)| (p.clone(), t.clone()))
        .collect();
    all_moves.sort_by_key(|(_, n)| -n.score * score_factor);

    // always get best move in the front of the list
    // if there are multiple moves with the same highest score, the selected one might not be plotted
    // not the most efficient way to do it, but this is not a hotspot
    let best_move_idx = all_moves
        .iter()
        .position(|m| m.0 == decision_tree.best_move.unwrap())
        .unwrap();
    if best_move_idx != 0 {
        all_moves.rotate_right(1);
        let new_best_idx = (best_move_idx + 1) % all_moves.len();
        all_moves.swap(0, new_best_idx);
    }
    let selected_alternatives = if draw_all_at_depth == depth {
        1000
    } else {
        alternatives_to_draw
    };

    // draw nodes ordered by score
    for (m, tree_node) in all_moves.into_iter().take(1 + selected_alternatives) {
        let new_game = game.apply_move(m);
        let child_node = graph_node(
            graph,
            tree_node,
            new_game,
            depth + 1,
            max_depth,
            node_id,
            alternatives_to_draw,
            draw_all_at_depth
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
