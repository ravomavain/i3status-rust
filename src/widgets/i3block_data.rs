/// Represent block as described in https://i3wm.org/docs/i3bar-protocol.html

#[derive(Debug, Clone)]
pub struct I3BlockData {
    pub full_text: String,
    pub short_text: Option<String>,
    pub color: Option<String>,
    pub background: Option<String>,
    pub border: Option<String>,
    pub border_top: Option<usize>,
    pub border_right: Option<usize>,
    pub border_bottom: Option<usize>,
    pub border_left: Option<usize>,
    pub min_width: Option<String>,
    pub align: Option<I3BlockAlign>,
    pub name: Option<String>,
    pub instance: Option<String>,
    pub urgent: Option<bool>,
    pub separator: Option<bool>,
    pub separator_block_width: Option<usize>,
    pub markup: Option<String>,
}

macro_rules! json_add_str {
    ($retval:ident, $obj:expr, $name:expr) => {
        if let Some(ref val) = $obj {
            $retval.push_str(&format!("\"{}\":\"{}\",", stringify!($name), val));
        }
    };
}
macro_rules! json_add_val {
    ($retval:ident, $obj:expr, $name:expr) => {
        if let Some(val) = $obj {
            $retval.push_str(&format!("\"{}\":{},", stringify!($name), val));
        }
    };
}

impl I3BlockData {
    pub fn render(&self) -> String {
        let mut retval = format!("{{\"full_text\":\"{}\",", self.full_text);

        json_add_str!(retval, self.short_text, short_text);
        json_add_str!(retval, self.color, color);
        json_add_str!(retval, self.background, background);
        json_add_str!(retval, self.border, border);
        json_add_val!(retval, self.border_top, border_top);
        json_add_val!(retval, self.border_right, border_right);
        json_add_val!(retval, self.border_bottom, border_bottom);
        json_add_val!(retval, self.border_left, border_left);
        json_add_str!(retval, self.min_width, min_width);
        match self.align {
            Some(I3BlockAlign::Center) => retval.push_str("align:\"center\","),
            Some(I3BlockAlign::Right) => retval.push_str("align:\"right\","),
            Some(I3BlockAlign::Left) => retval.push_str("align:\"left\","),
            None => {}
        }
        json_add_str!(retval, self.name, name);
        json_add_str!(retval, self.instance, instance);
        json_add_val!(retval, self.urgent, urgent);
        json_add_val!(retval, self.separator, separator);
        json_add_val!(retval, self.separator_block_width, separator_block_width);
        json_add_str!(retval, self.markup, markup);

        retval.push('}');
        retval
    }
}

impl Default for I3BlockData {
    fn default() -> Self {
        Self {
            full_text: String::new(),
            short_text: None,
            color: None,
            background: None,
            border: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
            min_width: None,
            align: None,
            name: None,
            instance: None,
            urgent: None,
            separator: Some(false),
            separator_block_width: Some(0),
            markup: Some("pango".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum I3BlockAlign {
    Center,
    Right,
    Left,
}