use crate::{consts, helpers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Widget},
};

pub fn instructions() -> Vec<Span<'static>> {
    vec![
        " ↑↓".bold().blue(),
        " Change grid".into(),
        "  ⏎".bold().blue(),
        " Select grid ".into(),
    ]
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PregameState {
    pub grid_size: usize,
}

impl Default for PregameState {
    fn default() -> Self {
        Self {
            grid_size: consts::MIN_GRID_SIZE,
        }
    }
}

pub struct PregameWidget(pub helpers::Rfc<PregameState>);

impl Widget for &PregameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (gwidth, gheight) = if area.width * 2 < area.height {
            let w = area.width as f64 * consts::PREGAME_GRID_SIZE;
            (w.round() as u16, (w / 2.).round() as u16)
        } else {
            let h = area.height as f64 * consts::PREGAME_GRID_SIZE;
            ((h * 2.).round() as u16, h.round() as u16)
        };
        let garea = helpers::center(
            area,
            Constraint::Length(gwidth),
            Constraint::Length(gheight),
        );
        let grid_size = (*self.0).borrow().grid_size;
        Block::default()
            .title(Line::from(format!("Choose your grid: {grid_size}x{grid_size}")).centered())
            .render(helpers::centered_scale(garea, 1.1, 1.1), buf);
        let rows = Layout::vertical((0..grid_size).map(|_| Constraint::Fill(1))).split(garea);
        for &row in rows.iter() {
            let cols = Layout::horizontal((0..grid_size).map(|_| Constraint::Fill(1))).split(row);
            for &col in cols.iter() {
                Block::bordered().border_set(border::PLAIN).render(col, buf);
            }
        }
    }
}
