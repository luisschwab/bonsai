use iced::Background::Color as BackgroundColor;
use iced::border::Radius;
use iced::theme::palette::Pair;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Theme};

use crate::common::interface::color::{BLACK, GREEN, OFF_WHITE, ORANGE, RED};
use crate::common::interface::container::common::{BORDER_RADIUS, BORDER_WIDTH, SHADOW};
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

#[derive(Debug, Clone)]
pub(crate) enum ActionButton {
    Start,
    Restart,
    Shutdown,
}

/// Style for `ACTION` buttons on the `NODE OVERVIEW` tab.
pub(crate) fn action_button(
    node_status: &NodeStatus,
    action_button: ActionButton,
) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, button_status| {
        let pair = match action_button {
            ActionButton::Start => Pair {
                color: GREEN,
                text: BLACK,
            },
            ActionButton::Restart => Pair {
                color: ORANGE,
                text: BLACK,
            },
            ActionButton::Shutdown => Pair {
                color: RED,
                text: BLACK,
            },
        };

        // Disable the button according to the current [`NodeStatus`].
        #[allow(clippy::match_like_matches_macro)]
        let should_disable = match (&node_status, &action_button) {
            (NodeStatus::Running, ActionButton::Start) => true,
            (NodeStatus::Inactive, ActionButton::Restart) => true,
            (NodeStatus::Inactive, ActionButton::Shutdown) => true,
            (NodeStatus::Starting, _) => true,
            (NodeStatus::ShuttingDown, _) => true,
            (NodeStatus::Failed(_), ActionButton::Restart) => true,
            (NodeStatus::Failed(_), ActionButton::Shutdown) => true,
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
