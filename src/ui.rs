use crate::app::{App, CurrentPane};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::Style,
    symbols,
    widgets::*,
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    ui(f, app);
    //    let chunks = Layout::horizontal([
    //        Constraint::Length(40),
    //        Constraint::Length(1),
    //        Constraint::Length(3),
    //    ])
    //    .split(f.size());
    //
    //    let title_block = Block::default()
    //        .borders(Borders::ALL)
    //        .title("Paths")
    //        .style(Style::default());
    //
    //    let path_block = Block::default()
    //        .borders(Borders::ALL)
    //        .title("Paths")
    //        .style(Style::new());
    //
    //    let left_chunks =
    //        Layout::vertical([Constraint::Length(50), Constraint::Length(50)]).split(f.size());
    //
    //    let title =
    //        Paragraph::new(Text::styled("Create new Json", Style::default())).block(title_block);
    //
    //    f.render_widget(path_block, left_chunks[0]);
    //    f.render_widget(title, chunks[0]);
}

// ANCHOR: ui
fn ui(frame: &mut Frame, app: &mut App) {
    // create a layout that splits the screen into 2 equal columns and the right column
    // into 2 equal rows

    // ANCHOR: layout
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        // use a 49/51 split instead of 50/50 to ensure that any extra space is on the right
        // side of the screen. This is important because the right side of the screen is
        // where the borders are collapsed.
        .constraints([Constraint::Percentage(26), Constraint::Percentage(74)])
        .split(frame.size());
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        // use a 49/51 split to ensure that any extra space is on the bottom
        .constraints([Constraint::Percentage(49), Constraint::Percentage(51)])
        .split(layout[0]);
    // ANCHOR_END: layout
    // ANCHOR: left_block
    frame.render_widget(
        Block::new()
            // don't render the right border because it will be rendered by the right block
            .border_set(symbols::border::ROUNDED)
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
            .title("Left Block"),
        layout[1],
    );
    // ANCHOR_END: left_block

    // ANCHOR: top_right_block
    // top right block must render the top left border to join with the left block
    let top_right_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.horizontal_down,
        ..symbols::border::PLAIN
    };

    let vertical_scroll = 0; // from app state

    let items: Vec<Line> = app
        .api_docs
        .paths
        .iter()
        .map(|p| Line::from(String::from(p.0)))
        .collect();
    let paragraph = Paragraph::new(items.clone())
        .scroll((vertical_scroll as u16, 0))
        .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scrollbar_state = ScrollbarState::new(items.len()).position(vertical_scroll);

    // Note we render the paragraph
    frame.render_widget(paragraph, sub_layout[0]);
    // and the scrollbar, those are separate widgets
    frame.render_stateful_widget(
        scrollbar,
        sub_layout[0].inner(&Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
    let border_style = if app.current_pane == CurrentPane::ApiPaths {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    frame.render_widget(
        Block::new()
            .border_set(top_right_border_set)
            // don't render the bottom border because it will be rendered by the bottom block
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .border_style(border_style)
            .title("Top Right Block"),
        sub_layout[0],
    );
    //   app.api_docs.paths.iter().for_each(|path| {
    //       let title_block = Block::default()
    //           .borders(Borders::ALL)
    //           .title("Paths")
    //           .style(Style::default());

    //       let title = Paragraph::new(Text::styled(path.0, Style::default())).block(title_block);
    //       frame.render_widget(title, sub_layout[0])
    //   });
    // ANCHOR_END: top_right_block

    // ANCHOR: bottom_right_block
    // bottom right block must render:
    // - top left border to join with the left block and top right block
    // - top right border to join with the top right block
    // - bottom left border to join with the left block
    let collapsed_top_and_left_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        bottom_left: symbols::line::NORMAL.horizontal_up,
        ..symbols::border::PLAIN
    };
    let iborder_style = if app.current_pane == CurrentPane::Collections {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    frame.render_widget(
        Block::new()
            .border_set(collapsed_top_and_left_border_set)
            .borders(Borders::ALL)
            .border_style(iborder_style)
            .title("Bottom Right Block"),
        sub_layout[1],
    );
    // ANCHOR_END: bottom_right_block
}
