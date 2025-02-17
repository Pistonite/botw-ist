use catppuccin::FlavorColors;

pub fn create_style_sheet() -> String {
    let mut output = String::new();
    create_style_sheet_for_flavor("latte", &catppuccin::PALETTE.latte.colors, &mut output);
    create_style_sheet_for_flavor("frappe", &catppuccin::PALETTE.frappe.colors, &mut output);
    create_style_sheet_for_flavor("macchiato", &catppuccin::PALETTE.macchiato.colors, &mut output);
    create_style_sheet_for_flavor("mocha", &catppuccin::PALETTE.mocha.colors, &mut output);
    output
}

pub fn create_style_sheet_for_flavor(flavor_name: &str, colors: &FlavorColors, output: &mut String) {

    let gray = colors.overlay2.hex;
    let yellow = colors.yellow.hex;
    let mauve = colors.mauve.hex;
    let peach = colors.peach.hex;
    let green = colors.green.hex;
    let red = colors.red.hex;
    let lavender = colors.lavender.hex;
    let blue = colors.blue.hex;

    let css = format!(r#"
.{0} .skybook-tt-Comment {{ color: {gray}; font-style: italic; }}
.{0} .skybook-tt-Symbol {{ color: {gray}; }}
.{0} .skybook-tt-Number {{ color: {peach}; }}
.{0} .skybook-tt-Command {{ color: {yellow}; }}
.{0} .skybook-tt-Keyword {{ color: {mauve}; }}
.{0} .skybook-tt-Word {{ color: {green}; }}
.{0} .skybook-tt-QuotedWord {{ color: {red}; }}
.{0} .skybook-tt-ItemLiteral {{ color: {red}; }}
.{0} .skybook-tt-Variable {{ color: {lavender}; }}
.{0} .skybook-tt-Type {{ color: {blue}; }}
.{0} .skybook-tt-Amount {{ color: {peach}; }}
"#, flavor_name);

    output.push_str(&css);
}
