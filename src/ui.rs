use crate::app::{ApiResponseResult, App, CurrentPane};
use ratatui::{
    layout::{Constraint, Direction, Layout, Size},
    prelude::*,
    style::{palette::tailwind, Style},
    symbols,
    widgets::*,
    Frame,
};
use tui_scrollview::ScrollView;

mod resultview;
pub fn draw(f: &mut Frame, app: &mut App) {
    ui(f, app);
}
const SCROLLVIEW_HEIGHT: u16 = 100;
// ANCHOR: ui
fn ui(frame: &mut Frame, app: &mut App) {
    // create a layout that splits the screen into 2 equal columns and the right column
    // into 2 equal rows

    // ANCHOR: layou
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

    match &app.current_path {
        Some(selected) => match app.environments.get(app.index_environment) {
            Some(environment) => {
                let sub_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Fill(2),
                    ])
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

                let selected_tab_index = app.index_method;
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

                if let Some(res) = &app.api_response {
                    let mut scroll_view =
                        ScrollView::new(Size::new(sub_layout[2].width, SCROLLVIEW_HEIGHT));

                    let width = if sub_layout[2].height < SCROLLVIEW_HEIGHT {
                        sub_layout[2].width - 10
                    } else {
                        sub_layout[2].width - 10
                    };
                    let txt = match res {
                        ApiResponseResult::Failure(err) => &err,
                        ApiResponseResult::Success(success) => &success.body,
                    };

                    let url1 = Paragraph::new(String::from(txt))
                        .block(
                            Block::new()
                                .title(format!("{}{}", environment.url, selected.path.clone()))
                                .borders(Borders::ALL),
                        )
                        .style(Style::new().white())
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: false });
                    scroll_view.render_widget(
                        url1,
                        Rect::new(0, 0, sub_layout[2].width, SCROLLVIEW_HEIGHT),
                    );
                    scroll_view.render(
                        sub_layout[2],
                        frame.buffer_mut(),
                        &mut app.scroll_view_state,
                    );
                }
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
    // Note we render the paragraph
    // and the scrollbar, those are separate widgets
    let border_style = if app.current_pane == CurrentPane::ApiPaths {
        Style::default().yellow()
    } else {
        Style::default().white()
    };
    render_paths(frame, &app.index_path, app, sub_layout[0]);
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

fn render_environments(f: &mut Frame, app: &App, render_area: Rect) {
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        // use a 49/51 split to ensure that any extra space is on the bottom
        .constraints([Constraint::Percentage(100)])
        .vertical_margin(1)
        .split(render_area);

    let _active_border_style = if app.current_pane == CurrentPane::FilterApi {
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

    let mut state = ListState::default().with_selected(Some(app.index_environment.clone()));
    f.render_stateful_widget(list, sub_layout[0], &mut state);
    // ANCHOR_END: bottom_right_block
}
fn render_paths(f: &mut Frame, selected_path: &usize, app: &App, render_area: Rect) {
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
    let paths_in_ui = app
        .filtered_paths
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
fn result_title(app: &App) -> impl Widget {
    let palette = tailwind::SLATE;
    let fg = palette.c900;
    let bg = palette.c300;
    let keys_fg = palette.c50;
    let keys_bg = palette.c600;
    Line::from(vec![
        "Tui-scrollview  ".into(),
        "  ↓ | ↑ | PageDown | PageUp | Home | End  "
            .fg(keys_fg)
            .bg(keys_bg),
        "  Quit: ".into(),
        " Esc ".fg(keys_fg).bg(keys_bg),
    ])
    .style((fg, bg))
}
