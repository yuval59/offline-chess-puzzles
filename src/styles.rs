use iced::widget::{button, container, text, radio, svg, text_input, scrollable, pick_list, checkbox, slider};
use iced::widget::slider::{Handle, HandleShape};
use iced::{application, Color};
use iced::theme::{Container, Radio, Svg, TextInput, Scrollable, PickList, Checkbox, Slider, Menu};
use iced::overlay::menu;
use iced_aw::style::tab_bar;
use iced_aw::style::TabBarStyles;

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PieceTheme {
    Cburnett,
    Alpha,
    California,
    Cardinal,
    Governor,
    Dubrovny,
    Gioco,
    Icpieces,
    Maestro,
    Staunty,
    Tatiana,
    Merida,
    FontAlpha,
}

impl PieceTheme {

    pub const ALL: [PieceTheme; 13] = [
        PieceTheme::Cburnett,
        PieceTheme::Alpha,
        PieceTheme::California,
        PieceTheme::Cardinal,
        PieceTheme::Governor,
        PieceTheme::Dubrovny,
        PieceTheme::Gioco,
        PieceTheme::Icpieces,
        PieceTheme::Maestro,
        PieceTheme::Staunty,
        PieceTheme::Tatiana,
        PieceTheme::Merida,
        PieceTheme::FontAlpha,
    ];
}

impl std::fmt::Display for PieceTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceTheme::Alpha => "alpha",
                PieceTheme::California => "california",
                PieceTheme::Cardinal => "cardinal",
                PieceTheme::Governor => "governor",
                PieceTheme::Dubrovny => "dubrovny",
                PieceTheme::Gioco => "gioco",
                PieceTheme::Icpieces => "icpieces",
                PieceTheme::Maestro => "maestro",
                PieceTheme::Staunty => "staunty",
                PieceTheme::Tatiana => "tatiana",
                PieceTheme::Merida => "merida",
                PieceTheme::FontAlpha => "Paper - chess alpha",
                _ => "cburnett",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Blue,
    Green,
    Brown,
    Purple,
    Grey,
    ColdGrey,
    BlueDark,
    GreenDark,
    BrownDark,
    PurpleDark,
    GreyDark,
    ColdGreyDark,
}

impl Theme {
    pub fn palette(&self) -> OCPPalette {
        match self {
            Self::Blue => OCPPalette::BLUE,
            Self::Green => OCPPalette::GREEN,
            Self::Brown => OCPPalette::BROWN,
            Self::Purple => OCPPalette::PURPLE,
            Self::Grey => OCPPalette::GREY,
            Self::ColdGrey => OCPPalette::COLD_GREY,
            Self::BlueDark => OCPPalette::BLUE_DARK,
            Self::GreenDark => OCPPalette::GREEN_DARK,
            Self::BrownDark => OCPPalette::BROWN_DARK,
            Self::PurpleDark => OCPPalette::PURPLE_DARK,
            Self::GreyDark => OCPPalette::GREY_DARK,
            Self::ColdGreyDark => OCPPalette::COLD_GREY_DARK,
        }
    }
    pub const ALL: [Theme; 12] = [
        Theme::Blue,
        Theme::Green,
        Theme::Brown,
        Theme::Purple,
        Theme::Grey,
        Theme::ColdGrey,
        Theme::BlueDark,
        Theme::GreenDark,
        Theme::BrownDark,
        Theme::PurpleDark,
        Theme::GreyDark,
        Theme::ColdGreyDark,
    ];
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Blue => "Blue",
                Theme::Green => "Green",
                Theme::Brown => "Brown",
                Theme::Purple => "Purple",
                Theme::Grey => "Grey",
                Theme::ColdGrey => "Cold Grey",
                Theme::BlueDark => "Blue - Dark Mode",
                Theme::GreenDark => "Green - Dark Mode",
                Theme::BrownDark => "Brown - Dark Mode",
                Theme::PurpleDark => "Purple - Dark Mode",
                Theme::GreyDark => "Grey - Dark Mode",
                Theme::ColdGreyDark => "Cold Grey - Dark",
            }
        )
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().container_bg,
            text_color: Color::BLACK,
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: None,
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = ButtonStyle;

    fn active(&self, style: &ButtonStyle) -> button::Appearance {
        let palette = self.palette();

        match style {
            ButtonStyle::LightSquare => {
                button::Appearance {
                    background: Some(iced::Background::Color(palette.light_square)),
                    ..Default::default()
                }
            }
            ButtonStyle::DarkSquare => {
                button::Appearance {
                    background: Some(iced::Background::Color(palette.dark_square)),
                    ..Default::default()
                }
            }
            ButtonStyle::SelectedLightSquare => {
                button::Appearance {
                    background: Some(iced::Background::Color(palette.selected_light_square)),
                    ..Default::default()
                }
            }
            ButtonStyle::SelectedDarkSquare => {
                button::Appearance {
                    background: Some(iced::Background::Color(palette.selected_dark_square)),
                    ..Default::default()
                }
            }
            ButtonStyle::Paper => {
                button::Appearance {
                    background: Some(iced::Background::Color(rgb!(245., 245., 245.))),
                    border_width: 0.,
                    border_color: iced::Color::BLACK,
                    text_color: rgb!(45., 45., 45.),
                    ..Default::default()
                }
            }
            ButtonStyle::SelectedPaper => {
                button::Appearance {
                    background: Some(iced::Background::Color(rgb!(180., 180., 180.))),
                    border_width: 0.,
                    border_color: iced::Color::BLACK,
                    ..Default::default()
                }
            }
            ButtonStyle::Normal => {
                button::Appearance {
                    border_width: 2.,
                    border_color: palette.dark_square,
                    background: Some(iced::Background::Color(palette.light_square)),
                    text_color: palette.tab_label,
                    ..Default::default()
                }
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Normal => {
                button::Appearance {
                    border_width: 2.,
                    border_color: self.palette().dark_square,
                    background: Some(iced::Background::Color(self.palette().dark_square)),
                    text_color: self.palette().label_selected,
                    ..Default::default()
                }
            }
            _ => self.active(style)
        }
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
                text_color: Some(self.palette().simple_text),
                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Color::WHITE,
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(self.palette().light_square),
            border_radius: 1.0.into(),
            border_width: 1.,
            border_color: self.palette().dark_square,
            icon_color: self.palette().simple_text,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(self.palette().light_square),
            border_radius: 1.0.into(),
            border_width: 1.,
            border_color: self.palette().dark_square,
            icon_color: self.palette().simple_text,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette().tab_label
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.palette().tab_label
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(self.palette().light_square),
            border_radius: 1.0.into(),
            border_width: 1.,
            border_color: self.palette().dark_square,
            icon_color: self.palette().simple_text,
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.palette().dark_square
    }
}

impl svg::StyleSheet for Theme {
    type Style = Svg;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: None,
        }
    }
}

impl tab_bar::StyleSheet for Theme {
    type Style = TabBarStyles;

    fn active(&self, _style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
        let bg = if is_active { self.palette().dark_square } else { self.palette().light_square };
        let label_color = if is_active { self.palette().label_selected } else { self.palette().tab_label };
        tab_bar::Appearance {
            background: Some(iced::Background::Color(bg)),
            tab_label_background: iced::Background::Color(bg),
            text_color: label_color,
            icon_color: label_color,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_active: bool) -> tab_bar::Appearance {
        tab_bar::Appearance {
            tab_label_background: iced::Background::Color(self.palette().dark_square),
            text_color: self.palette().label_selected,
            icon_color: self.palette().label_selected,
            ..Default::default()
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(self.palette().light_square)),
            border_radius: 1.0.into(),
            border_width: 1.,
            border_color: self.palette().light_square,
            scroller: scrollable::Scroller
                {
                    color: self.palette().dark_square,
                    border_radius: 0.0.into(),
                    border_width: 1.,
                    border_color: self.palette().light_square,
                }
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(self.palette().light_square)),
            border_radius: 1.0.into(),
            border_width: 1.,
            border_color: self.palette().dark_square,
            scroller: scrollable::Scroller
                {
                    color: self.palette().dark_square,
                    border_radius: 1.0.into(),
                    border_width: 1.,
                    border_color: Color::BLACK,
                }
        }
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = Checkbox;

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: iced::Background::Color(self.palette().light_square),
            border_radius: 1.0.into(),
            border_width: 1.,
            border_color: Color::BLACK,
            icon_color: self.palette().tab_label,
            text_color: Some(self.palette().tab_label),
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: iced::Background::Color(self.palette().dark_square),
            border_radius: 1.0.into(),
            border_width: 1.,
            border_color: Color::BLACK,
            icon_color: self.palette().label_selected,
            text_color: Some(self.palette().label_selected),
        }
    }

}

impl pick_list::StyleSheet for Theme {
    type Style = PickList;

    fn active(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.palette().tab_label,
            placeholder_color: self.palette().tab_label,
            background: iced::Background::Color(self.palette().light_square),
            border_radius: 0.5.into(),
            border_width: 1.,
            border_color: self.palette().dark_square,
            handle_color: self.palette().dark_square
        }

    }

    fn hovered(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.palette().label_selected,
            placeholder_color: self.palette().label_selected,
            background: iced::Background::Color(self.palette().dark_square),
            border_radius: 0.5.into(),
            border_width: 1.,
            border_color: self.palette().dark_square,
            handle_color: self.palette().dark_square
        }
    }
}

impl menu::StyleSheet for Theme {
    type Style = Menu;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: self.palette().tab_label,
            background: iced::Background::Color(self.palette().light_square),
            border_radius: 0.3.into(),
            border_width: 1.,
            border_color: self.palette().dark_square,
            selected_text_color: self.palette().label_selected,
            selected_background: iced::Background::Color(self.palette().dark_square),
        }
    }
}

impl slider::StyleSheet for Theme {
    type Style = Slider;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (
                    Color::BLACK, Color::BLACK,
                ),
                width: 2.0,
                border_radius: 2.0.into(),
            },
            handle: Handle {
                shape: HandleShape::Rectangle { width: 7, border_radius: 1.0.into() },
                color: self.palette().light_square,
                border_width: 2.,
                border_color: self.palette().dark_square
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (
                    Color::BLACK, Color::BLACK,
                ),
                width: 2.0,
                border_radius: 2.0.into(),
            },
            handle: Handle {
                shape: HandleShape::Rectangle { width: 10, border_radius: 1.0.into() },
                color: self.palette().dark_square,
                border_width: 2.,
                border_color: self.palette().light_square
            },
        }
    }

    fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (
                    Color::BLACK, Color::BLACK,
                ),
                width: 2.0,
                border_radius: 2.0.into(),
            },
            handle: Handle {
                shape: HandleShape::Rectangle { width: 10, border_radius: 1.0.into() },
                color: self.palette().dark_square,
                border_width: 2.,
                border_color: self.palette().light_square
            },
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = Radio;

    fn active(&self, _style: &Radio, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: iced::Background::Color(self.palette().light_square),
            dot_color: self.palette().tab_label,
            border_width: 1.,
            border_color: self.palette().dark_square,
            text_color: Some(self.palette().simple_text),
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: iced::Background::Color(self.palette().dark_square),
            dot_color: self.palette().label_selected,
            border_width: 1.,
            border_color: self.palette().simple_text,
            text_color: Some(self.palette().simple_text),
        }
    }

}

/// Offline Chess Puzzles Palette
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OCPPalette {
    pub container_bg: Color,
    pub simple_text: Color,
    pub label_selected: Color,
    pub light_square: Color,
    pub dark_square: Color,
    pub selected_light_square: Color,
    pub selected_dark_square: Color,
    pub tab_label: Color,
}

impl OCPPalette {
    pub const BLUE: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(235.0, 249.0, 255),
        dark_square: rgb!(110.0, 174.0, 213.0),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };

    pub const BLUE_DARK: Self = Self {
        container_bg: rgb!(70.,99.,117.),
        light_square: rgb!(235.0, 249.0, 255),
        dark_square: rgb!(110.0, 174.0, 213.0),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREEN_DARK: Self = Self {
        container_bg: rgb!(87.,99.,76.),
        light_square: rgb!(238.0, 240.0, 203.0),
        dark_square: rgb!(136.0, 161.0, 111.0),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const BROWN_DARK: Self = Self {
        container_bg: rgb!(116.,99.,86.),
        light_square: rgb!(241., 221., 186.),
        dark_square: rgb!(186., 142., 107.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const PURPLE_DARK: Self = Self {
        container_bg: rgb!(89.,77.,101.),
        light_square: rgb!(233., 223., 242.),
        dark_square: rgb!(162., 136., 188.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const COLD_GREY_DARK: Self = Self {
        container_bg: rgb!(90.,90.,90.),
        light_square: rgb!(220., 220., 220.),
        dark_square: rgb!(140., 140., 140.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREY_DARK: Self = Self {
        container_bg: rgb!(71.,86.,92.),
        light_square: rgb!(222., 227., 230.),
        dark_square: rgb!(140., 162., 173.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::WHITE,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREEN: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(238.0, 240.0, 203.0),
        dark_square: rgb!(136.0, 161.0, 111.0),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const BROWN: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(241., 221., 186.),
        dark_square: rgb!(186., 142., 107.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const PURPLE: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(233., 223., 242.),
        dark_square: rgb!(162., 136., 188.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const COLD_GREY: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(220., 220., 220.),
        dark_square: rgb!(140., 140., 140.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
    pub const GREY: Self = Self {
        container_bg: Color::WHITE,
        light_square: rgb!(222., 227., 230.),
        dark_square: rgb!(140., 162., 173.),
        selected_light_square: rgb!(205,210,106),
        selected_dark_square: rgb!(170,162,58),
        simple_text: Color::BLACK,
        label_selected: Color::WHITE,
        tab_label: Color::BLACK,
    };
}

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Normal,
    LightSquare,
    DarkSquare,
    SelectedLightSquare,
    SelectedDarkSquare,
    Paper,
    SelectedPaper,
}
