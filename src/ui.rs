use crate::{
    api::Action,
    app::{App, AppApiPaths, CurrentPane},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::*,
    style::Style,
    symbols,
    widgets::*,
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    if let Some(_) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);
    }
    ui(f, app);
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
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(layout[0]);

    let paths = app.filter();
    match paths.get(app.cursor_path) {
        Some(selected) => match app.environments.get(app.selected_environment) {
            Some(environment) => {
                let sub_layout = Layout::default()
                    .direction(Direction::Vertical)
                    // use a 49/51 split to ensure that any extra space is on the bottom
                    .constraints([Constraint::Length(3), Constraint::Fill(2)])
                    .split(layout[1]);
                let text = vec![Line::from(vec![Span::styled(
                    format!("{}{}", environment.url, selected.path.clone()),
                    Style::new().green().italic(),
                )])];
                let url = Paragraph::new(text)
                    .block(
                        Block::new()
                            .title(format!("{}{}", environment.url, selected.path.clone()))
                            .borders(Borders::ALL),
                    )
                    .style(Style::new().white())
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                frame.render_widget(url, sub_layout[0]);

                let selected_tab_index = app.selected_method;
                let tabs = selected
                    .methods
                    .iter()
                    .map(|m| m.0.clone())
                    .collect::<Tabs>()
                    .select(selected_tab_index)
                    .highlight_style(Style::default().yellow())
                    .padding(" ", " ")
                    .divider(symbols::DOT);
                frame.render_widget(tabs, sub_layout[1]);
            }
            None => {}
        },
        None => {}
    };
    let active_border_style = if app.current_pane == CurrentPane::HttpCalls {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    frame.render_widget(
        Block::new()
            // don't render the right border because it will be rendered by the right block
            .borders(Borders::ALL)
            .border_style(active_border_style)
            .style(Style::default())
            .title("Left Block"),
        layout[1],
    );
    // Note we render the paragraph
    // and the scrollbar, those are separate widgets
    let border_style = if app.current_pane == CurrentPane::ApiPaths {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    render_paths(frame, &app.cursor_path, &paths, app, sub_layout[0]);
    frame.render_widget(
        Block::new()
            // don't render the bottom border because it will be rendered by the bottom block
            .borders(Borders::ALL)
            .border_style(border_style)
            .title("API Paths"),
        sub_layout[0],
    );
    let iborder_style = if app.current_pane == CurrentPane::Collections {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    render_environments(frame, app, sub_layout[1]);
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_style(iborder_style)
            .title("Environments"),
        sub_layout[1],
    );
}

fn addmethod<'a>(method: &Option<Action>, tabs: &'a mut Vec<&'a str>, tabname: &'a str) {
    match method {
        Some(_) => tabs.push(tabname),
        _ => (),
    };
}

fn render_environments(f: &mut Frame, app: &App, render_area: Rect) {
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        // use a 49/51 split to ensure that any extra space is on the bottom
        .constraints([Constraint::Percentage(100)])
        .vertical_margin(1)
        .split(render_area);

    let active_border_style = if app.current_pane == CurrentPane::FilterApi {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    let paths_in_ui = app
        .environments
        .iter()
        .map(|p| format!("{}", p.name))
        .collect::<List>();

    let list = paths_in_ui
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().cyan().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .scroll_padding(1)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let mut state = ListState::default().with_selected(Some(app.selected_environment.clone()));
    f.render_stateful_widget(list, sub_layout[0], &mut state);
    // ANCHOR_END: bottom_right_block
}
fn render_paths(
    f: &mut Frame,
    selected_path: &usize,
    paths: &Vec<&AppApiPaths>,
    app: &App,
    render_area: Rect,
) {
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        // use a 49/51 split to ensure that any extra space is on the bottom
        .constraints([Constraint::Length(3), Constraint::Fill(2)])
        .split(render_area);

    let active_border_style = if app.current_pane == CurrentPane::FilterApi {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    let text = vec![Line::from(vec![
        Span::styled("Filter: ", Style::new().green().italic()),
        app.filter.clone().into(),
    ])];

    let url = Paragraph::new(text)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(active_border_style),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(url, sub_layout[0]);
    let paths_in_ui = paths
        .iter()
        .map(|p| format!("{}", p.path))
        .collect::<List>();

    let list = paths_in_ui
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().cyan().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .scroll_padding(1)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let mut state = ListState::default().with_selected(Some(selected_path.clone()));
    f.render_stateful_widget(list, sub_layout[1], &mut state);
    // ANCHOR_END: bottom_right_block
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
