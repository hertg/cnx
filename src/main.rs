use anyhow::Error;
use cnx::text::*;
use cnx::widgets::*;
use cnx::{Cnx, Position};

fn main() -> Result<(), Error> {
    let attr = Attributes {
        font: Font::new("monospace"),
        fg_color: Color::white(),
        bg_color: Some(Color::black()),
        padding: Padding::new(8.0, 8.0, 8.0, 8.0),
    };

    let pager_attr = Attributes {
        font: Font::new("monospace"),
        fg_color: Color::white(),
        bg_color: Some(Color::blue()),
        padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    };

    let mut p2_attr = pager_attr.clone();
    p2_attr.bg_color = None;

    let mut cnx = Cnx::new(Position::Top);
    cnx.add_widget(Pager::new(pager_attr, p2_attr));
    cnx.add_widget(ActiveWindowTitle::new(attr.clone()));
    cnx.add_widget(Clock::new(attr.clone(), Option::None));
    cnx.run()?;

    Ok(())
}