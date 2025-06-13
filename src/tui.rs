use std::path::PathBuf;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{
        Block, BorderType, Cell, Clear, HighlightSpacing, Paragraph, Row, Table, TableState,
    },
};

const INFO_TEXT: &str = "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right";

struct TableColors {
    selected_row_style_fg: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new() -> Self {
        Self {
            selected_row_style_fg: Color::Blue,
            footer_border_color: Color::LightBlue,
        }
    }
}

pub struct App {
    state: TableState,
    items: Vec<PathBuf>,
    colors: TableColors,
    show_popup: bool,
}

impl App {
    pub fn new(paths: &Vec<PathBuf>) -> Self {
        Self {
            state: TableState::default().with_selected(0),
            colors: TableColors::new(),
            items: paths.clone(),
            show_popup: false,
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => {
                        if !self.show_popup {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                    KeyCode::Char('d') => {
                        self.show_popup = true;
                    }
                    KeyCode::Esc => {
                        self.show_popup = false;
                    }
                    KeyCode::Char('n') => {
                        self.show_popup = false;
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.area());

        self.render_table(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);

        let header = ["Path"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = self
            .items
            .iter()
            .enumerate()
            .map(|(_, data)| Row::new(vec![Cell::from(Text::from(data.to_string_lossy()))]));
        let bar = " █ ";

        let t = Table::new(rows, [Constraint::Percentage(100)])
            .header(header)
            .row_highlight_style(selected_row_style)
            .highlight_symbol(bar)
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(t, area, &mut self.state);

        if self.show_popup {
            let selected_index = self.state.selected();
            if selected_index.is_none() {
                return;
            }
            let selected_index = selected_index.unwrap();
            let selected_item = &self.items[selected_index];

            let popup_area = popup_area(area, 80, 50);
            let widget = Paragraph::new(selected_item.to_string_lossy())
                .centered()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(self.colors.footer_border_color)),
                );

            frame.render_widget(Clear, popup_area);
            frame.render_widget(widget, popup_area);
        }
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(INFO_TEXT).centered().block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(self.colors.footer_border_color)),
        );

        frame.render_widget(info_footer, area);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
