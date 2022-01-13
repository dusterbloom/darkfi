// list on left
// info page on right
// when selected takes up full page
use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let slice = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let nodes: Vec<ListItem> = app
        .node_list
        .nodes
        .iter()
        .map(|(id, info)| {
            let line1 = Spans::from(id.to_string());
            let line2 = Spans::from(info.to_string());

            ListItem::new(vec![line1, line2]).style(Style::default())
        })
        .collect();

    let nodes = List::new(nodes)
        .block(Block::default())
        .highlight_style(Style::default().bg(Color::Black).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(nodes, slice[0], &mut app.node_list.state);

    let mut info_vec = Vec::new();
    for val in app.node_list.nodes.values() {
        info_vec.push(Spans::from(val.to_string()))
    }

    let graph = Paragraph::new(info_vec)
        .block(Block::default())
        .style(Style::default());

    // TODO: make this a stateful widget that changes with scroll
    f.render_widget(graph, slice[1]);
}