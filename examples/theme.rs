use cliclack::{input, intro, log, outro, set_theme, Theme, ThemeState};
use console::{style, Style};

struct MagentaTheme;

impl Theme for MagentaTheme {
    fn bar_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().magenta(),
            ThemeState::Error(_) => Style::new().red(),
            _ => Style::new().magenta().dim(),
        }
    }

    fn state_symbol_color(&self, _state: &ThemeState) -> Style {
        Style::new().magenta()
    }

    fn info_symbol(&self) -> String {
        "⚙".into()
    }
}

fn main() -> std::io::Result<()> {
    // Set a no-op Ctrl-C handler so that Ctrl-C results in a
    // `term.read_key()` error instead of terminating the process. You can skip
    // this step if you have your own Ctrl-C handler already set up.
    //
    // We cannot (easily) handle this at the library level due to
    // https://github.com/Detegr/rust-ctrlc/issues/106#issuecomment-1887793468.
    ctrlc::set_handler(move || {}).expect("Error setting Ctrl-C handler");

    set_theme(MagentaTheme);

    intro(style(" theme ").on_magenta().black())?;

    let path: String = input("Where should we create your project?")
        .placeholder("./right-here")
        .interact()?;

    log::info(format!("Project path: {path}"))?;

    outro("Done")?;

    Ok(())
}
