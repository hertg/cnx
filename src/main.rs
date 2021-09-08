use anyhow::Error;
use cnx::text::*;
use cnx::widgets::*;
use cnx::{Cnx, Position};

fn main() -> Result<(), Error> {
    let attr = Attributes {
        font: Font::new("Envy Code R 21"),
        fg_color: Color::white(),
        bg_color: None,
        padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    };

    let mut cnx = Cnx::new(Position::Top);
    cnx.add_widget(ActiveWindowTitle::new(attr.clone()));
    cnx.add_widget(Clock::new(attr.clone(), Option::None));
    cnx.run()?;

    Ok(())
}