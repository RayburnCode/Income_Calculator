<!-- @format -->

# Theme Usage Guide

Your desktop application now has a simple Light/Dark theme system! Here's how to use it:

## How It Works

The theme system uses CSS variables that automatically update when you toggle between light and dark modes. The `ThemeToggle` button in your navbar controls this.

## Available CSS Variables

The following CSS variables are available for use in your components:

- `--bg-primary`: Main background color
- `--bg-secondary`: Secondary background (cards, panels)
- `--bg-tertiary`: Tertiary background (hover states)
- `--text-primary`: Primary text color
- `--text-secondary`: Secondary text color
- `--text-tertiary`: Tertiary text (disabled, muted)
- `--border-color`: Border colors
- `--accent-color`: Accent/brand color (buttons, links)
- `--accent-hover`: Accent hover state

## Usage in Components

### Using inline styles (recommended for desktop):

```rust
rsx! {
    div {
        style: "background: var(--bg-primary); color: var(--text-primary);",
        "This will automatically switch between light and dark!"
    }
}
```

### Example: Styled Card Component

```rust
#[component]
pub fn Card(title: String, children: Element) -> Element {
    rsx! {
        div {
            style: "
                background: var(--bg-secondary);
                color: var(--text-primary);
                border: 1px solid var(--border-color);
                border-radius: 8px;
                padding: 16px;
            ",
            h2 {
                style: "color: var(--text-primary); font-size: 18px; font-weight: bold; margin-bottom: 8px;",
                "{title}"
            }
            div {
                style: "color: var(--text-secondary);",
                {children}
            }
        }
    }
}
```

### Example: Button Component

```rust
#[component]
pub fn Button(onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    rsx! {
        button {
            onclick: move |e| onclick.call(e),
            style: "
                background: var(--accent-color);
                color: white;
                padding: 8px 16px;
                border-radius: 6px;
                border: none;
                cursor: pointer;
                transition: background 0.2s;
            ",
            onmouseenter: |_| {
                // Could use :hover in CSS or handle programmatically
            },
            {children}
        }
    }
}
```

## Accessing Theme State in Code

If you need to check the current theme in your Rust code:

```rust
use crate::components::theme::{use_theme, Theme};

#[component]
pub fn MyComponent() -> Element {
    let theme_context = use_theme();
    let is_dark = matches!(*theme_context.theme.read(), Theme::Dark);

    rsx! {
        div {
            "Current theme is: {if is_dark { \"Dark\" } else { \"Light\" }}"
        }
    }
}
```

## The ThemeToggle Button

The `ThemeToggle` component is already added to your navbar. It shows:

- A moon icon when in light mode (click to go dark)
- A sun icon when in dark mode (click to go light)

## Customizing Colors

To customize the theme colors, edit the CSS variables in [frontend/src/components/theme.rs](frontend/src/components/theme.rs):

```rust
// For dark theme
if is_dark {
    r#"
        :root {
            --bg-primary: #1a1a1a;      // Change these values
            --bg-secondary: #2d2d2d;
            // ... etc
        }
    "#
} else {
    // For light theme
    r#"
        :root {
            --bg-primary: #ffffff;      // Change these values
            --bg-secondary: #f9fafb;
            // ... etc
        }
    "#
}
```

## Tips

1. **Use CSS variables for all colors** instead of hardcoded Tailwind classes for theme-aware styling
2. **The theme automatically persists** during the session (changes when you toggle)
3. **Transitions are smooth** - add `transition` CSS properties for animated color changes
4. **Keep it simple** - This is designed to be straightforward for desktop apps

## Testing

To test the theme:

1. Run your app: `dx serve --platform desktop`
2. Look for the moon/sun icon in your navbar
3. Click it to toggle between light and dark modes
4. All components using CSS variables will update automatically!
