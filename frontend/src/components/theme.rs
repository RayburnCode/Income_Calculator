use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: Signal<Theme>,
}

impl ThemeContext {
    pub fn toggle(&mut self) {
        let current = *self.theme.read();
        let new_theme = match current {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
        self.theme.set(new_theme);
    }

    pub fn is_dark(&self) -> bool {
        matches!(*self.theme.read(), Theme::Dark)
    }
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
}

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let theme = use_signal(|| Theme::Light);

    use_context_provider(|| ThemeContext { theme });

    let class = if matches!(*theme.read(), Theme::Dark) {
        "dark min-h-screen"
    } else {
        "min-h-screen"
    };

    rsx! {
        div { class, {children} }
    }
}