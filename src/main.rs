use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, 
    event,
};

fn main() -> std::io::Result<()> {
    
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Hello World"),
        ResetColor
    )?;    
    Ok(())
}
