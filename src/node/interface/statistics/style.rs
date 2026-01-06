use iced::Background::Color as BackgroundColor;
use iced::Border;
use iced::Theme;
use iced::border::Radius;
use iced::theme::palette::Pair;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;

use crate::common::interface::color::BLACK;
use crate::common::interface::color::GREEN;
use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::color::ORANGE;
use crate::common::interface::color::RED;
use crate::common::interface::container::common::BORDER_RADIUS;
use crate::common::interface::container::common::BORDER_WIDTH;
use crate::common::interface::container::common::SHADOW;
use crate::node::control::NodeStatus;

/// Container for displaying logs through a `tracing_subscriber`.
pub(crate) fn log_container() -> impl Fn(&Theme) -> ContainerStyle {
    |_theme| ContainerStyle {
        border: Border {
            color: OFF_WHITE,
            width: BORDER_WIDTH,
            radius: Radius::new(BORDER_RADIUS),
        },
        shadow: SHADOW,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ControlButton {
    Start,
    Restart,
    Shutdown,
}

/// Style for `ACTION` buttons on the `NODE OVERVIEW` tab.
pub(crate) fn action_button(
    node_status: &NodeStatus,
    action_button: ControlButton,
) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, button_status| {
        let pair = match action_button {
            ControlButton::Start => Pair {
                color: GREEN,
                text: BLACK,
            },
            ControlButton::Restart => Pair {
                color: ORANGE,
                text: BLACK,
            },
            ControlButton::Shutdown => Pair {
                color: RED,
                text: BLACK,
            },
        };

        // Disable the button according to the current [`NodeStatus`].
        #[allow(clippy::match_like_matches_macro)]
        let should_disable = match (&node_status, &action_button) {
            (NodeStatus::Running, ControlButton::Start) => true,
            (NodeStatus::Inactive, ControlButton::Restart) => true,
            (NodeStatus::Inactive, ControlButton::Shutdown) => true,
            (NodeStatus::Starting, _) => true,
            (NodeStatus::ShuttingDown, _) => true,
            (NodeStatus::Failed(_), ControlButton::Restart) => true,
            (NodeStatus::Failed(_), ControlButton::Shutdown) => true,
            _ => false,
        };

        let pair = if should_disable || button_status == ButtonStatus::Disabled {
            Pair {
                color: pair.color.scale_alpha(0.5),
                text: pair.text.scale_alpha(0.5),
            }
        } else {
            match button_status {
                ButtonStatus::Active => pair,
                ButtonStatus::Hovered | ButtonStatus::Pressed => Pair {
                    color: pair.color.scale_alpha(0.8),
                    text: pair.text,
                },
                ButtonStatus::Disabled => pair,
            }
        };

        ButtonStyle {
            background: Some(BackgroundColor(pair.color)),
            text_color: pair.text,
            border: Border {
                color: BLACK,
                width: BORDER_WIDTH,
                radius: Radius::new(BORDER_RADIUS),
            },
            ..ButtonStyle::default()
        }
    }
}
