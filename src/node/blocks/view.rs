use bitcoin::Amount;
use bitcoin::Block;
use iced::Alignment::Center;
use iced::Element;
use iced::Length;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::scrollable::Scrollbar;
use iced::widget::text;
use iced::widget::text_input;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::constants::CELL_HEIGHT;
use crate::common::interface::container::button_container;
use crate::common::interface::container::shadow_container;
use crate::common::interface::font::BERKELEY_MONO_BOLD;
use crate::common::util::format_btc;
use crate::common::util::format_bytes;
use crate::common::util::format_thousands;
use crate::common::util::parse_formatted_u32;
use crate::common::util::split_hash_64;
use crate::node::blocks::transaction_view::transactions_table;
use crate::node::message::NodeMessage;
use crate::node::style::input_field;
use crate::node::style::table_cell;
use crate::node::style::title_container;
use crate::node::style::transparent_button;

/// Get the block subsidy in satoshis based on blockheight.
fn get_block_subsidy(height: u32) -> u64 {
    const SUBSIDY_HALVING_INTERVAL: u32 = 210_000; // Blocks.
    const INITIAL_SUBSIDY: u64 = 50 * 100_000_000; // 50 BTC in satoshis.

    let halvings = height / SUBSIDY_HALVING_INTERVAL;

    if halvings >= 64 {
        return 0;
    }

    INITIAL_SUBSIDY >> halvings
}

fn view_latest_blocks<'a>(latest_blocks: &'a [Block]) -> Column<'a, NodeMessage> {
    let latest_title: Container<'_, NodeMessage> = container(text("LATEST BLOCKS").size(24));
    let latest_canvas: Container<'_, NodeMessage> = {
        let blocks_column = latest_blocks.iter().take(5).enumerate().fold(
            column![].spacing(0),
            |col, (idx, block)| {
                let block_height = block.bip34_block_height().unwrap_or(0);
                let tx_count = block.txdata.len();
                let block_size_bytes = bitcoin::consensus::encode::serialize(&block).len();
                let block_size = format_bytes(block_size_bytes);

                let ascii: &[&str] = &[
                    "  ___________  ",
                    " /          /| ",
                    "/__________/ | ",
                    "|          | | ",
                    "|          | | ",
                    "|          | / ",
                    "|__________|/  ",
                ];
                let cube = text(ascii.join("\n")).font(BERKELEY_MONO_BOLD).size(10);

                let block_info = row![
                    cube,
                    column![
                        text(format!("BLOCK {}", format_thousands(block_height))).size(12),
                        text(format!("{} TRANSACTION(S)", tx_count)).size(12),
                        text(block_size).size(12),
                    ]
                    .spacing(2)
                ]
                .spacing(10)
                .align_y(Center);

                let block_button = button(container(block_info).padding(10))
                    .style(transparent_button())
                    .on_press(NodeMessage::BlockExplorerHeightUpdate(block_height));

                let col = col.push(block_button);

                if idx < 4 {
                    col.push(Space::new().height(Length::Fill))
                } else {
                    col
                }
            },
        );

        container(blocks_column)
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Center)
            .style(title_container())
    };

    column![latest_title, latest_canvas].spacing(5)
}

fn explorer_title<'a>(
    block_height: &'a str,
    current_height: Option<u32>,
) -> Container<'a, NodeMessage> {
    container(row![
        text("BLOCK EXPLORER").size(24),
        Space::new().width(Length::Fill),
        row![
            button(
                text("<")
                    .size(16)
                    .font(BERKELEY_MONO_BOLD)
                    .align_x(Center)
                    .align_y(Center)
            )
            .on_press_maybe(
                current_height
                    .and_then(|h| h.checked_sub(1))
                    .map(|h| NodeMessage::BlockExplorerHeightUpdate(h as u64))
            )
            .style(button_container())
            .padding(10)
            .height(CELL_HEIGHT),
            container(
                text_input("", block_height)
                    .on_input(NodeMessage::BlockHeightInputChanged)
                    .style(input_field())
                    .size(16)
                    .padding(10)
                    .align_x(Center)
                    .width(Length::Fixed(110.0))
            )
            .padding(0)
            .style(shadow_container())
            .height(CELL_HEIGHT),
            button(
                text(">")
                    .size(16)
                    .font(BERKELEY_MONO_BOLD)
                    .align_x(Center)
                    .align_y(Center)
            )
            .on_press_maybe(
                current_height
                    .and_then(|h| h.checked_add(1))
                    .map(|h| NodeMessage::BlockExplorerHeightUpdate(h as u64))
            )
            .style(button_container())
            .padding(10)
            .height(CELL_HEIGHT)
        ]
        .spacing(10)
    ])
}

fn block_header_table<'a>(
    current_block: &'a Option<Block>,
    current_height: Option<u32>,
) -> Column<'a, NodeMessage> {
    let version = current_block.as_ref().map_or(String::new(), |b| {
        format!("{:08x}", b.header.version.to_consensus())
    });
    let time = current_block
        .as_ref()
        .map_or(String::new(), |b| b.header.time.to_string());
    let bits = current_block
        .as_ref()
        .map_or(String::new(), |b| format!("{:08x}", b.header.bits));
    let nonce = current_block
        .as_ref()
        .map_or(String::new(), |b| format!("{:08x}", b.header.nonce));
    let prev_blockhash = current_block.as_ref().map_or(String::new(), |b| {
        split_hash_64(b.header.prev_blockhash.to_string())
    });
    let merkle_root = current_block.as_ref().map_or(String::new(), |b| {
        split_hash_64(b.header.merkle_root.to_string())
    });

    let (block_size, block_weight, subsidy_and_fees, total_moved) = current_block.as_ref().map_or(
        (String::new(), String::new(), String::new(), String::new()),
        |block| {
            let block_size_bytes = bitcoin::consensus::encode::serialize(&block).len();
            let block_size = format_bytes(block_size_bytes);

            let block_weight = format!("{} WU", format_thousands(block.weight().to_wu() as u32));

            // Need to fetch all prevouts for fees (too network intensive?)
            let fees = Amount::from_sat(0);
            let subsidy = Amount::from_sat(get_block_subsidy(current_height.unwrap_or(0)));
            let subsidy_and_fees = format_btc(subsidy + fees);

            let mut total_moved = Amount::from_sat(0);
            for tx in &block.txdata {
                let output_sum: u64 = tx.output.iter().map(|o| o.value.to_sat()).sum();
                total_moved += Amount::from_sat(output_sum);
            }

            let total_moved = format_btc(total_moved);

            (block_size, block_weight, subsidy_and_fees, total_moved)
        },
    );

    column![
        row![
            container(text("HEADER & STATS").font(BERKELEY_MONO_BOLD).size(16))
                .width(Length::Fill)
                .align_y(Center)
                .align_x(Center)
                .height(CELL_HEIGHT)
                .style(table_cell()),
        ]
        .spacing(0),
        row![
            container(text("VERSION").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(version).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .align_y(Center)
                .style(table_cell()),
            container(text("TIME").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(time).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .align_y(Center)
                .style(table_cell()),
        ]
        .spacing(0),
        row![
            container(text("PREV BLOCKHASH").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(prev_blockhash).size(9))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .align_y(Center)
                .align_x(Center)
                .style(table_cell()),
            container(text("BITS").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(bits).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .align_y(Center)
                .style(table_cell()),
        ]
        .spacing(0),
        row![
            container(text("MERKLE ROOT").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(merkle_root).size(9))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .align_y(Center)
                .align_x(Center)
                .style(table_cell()),
            container(text("NONCE").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(nonce).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .align_y(Center)
                .style(table_cell()),
        ]
        .spacing(0),
        row![
            container(text("BLOCK SIZE").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(block_size).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .align_y(Center)
                .style(table_cell()),
            container(text("TOTAL MOVED").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(total_moved).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .align_y(Center)
                .style(table_cell()),
        ]
        .spacing(0),
        row![
            container(text("BLOCK WEIGHT").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(block_weight).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text("SUBSIDY + FEES").font(BERKELEY_MONO_BOLD).size(12))
                .width(Length::FillPortion(2))
                .height(CELL_HEIGHT)
                .padding(10)
                .style(table_cell()),
            container(text(subsidy_and_fees).size(12))
                .width(Length::FillPortion(3))
                .height(CELL_HEIGHT)
                .padding(10)
                .align_y(Center)
                .style(table_cell()),
        ]
        .spacing(0),
    ]
    .spacing(0)
}

pub fn view_blocks<'a>(
    block_height: &'a str,
    latest_blocks: &'a [Block],
    current_block: &'a Option<Block>,
    expanded_tx_idx: &'a Option<usize>,
    last_action_error: Option<&'a str>,
) -> Element<'a, NodeMessage> {
    let left = column![container(view_latest_blocks(latest_blocks))]
        .spacing(20)
        .width(Length::FillPortion(1));

    // Parse the `block_height` string into a `u32`.
    let current_height = parse_formatted_u32(block_height);
    let explorer_title = explorer_title(block_height, current_height);
    let header_table = block_header_table(current_block, current_height);

    let transactions_scrollable = scrollable(transactions_table(current_block, expanded_tx_idx))
        .height(Length::Fill)
        .direction(iced::widget::scrollable::Direction::Vertical(
            Scrollbar::hidden(),
        ));

    let explorer_canvas = container(column![header_table, transactions_scrollable,].spacing(0))
        .padding(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(title_container());

    let explorer_error = text(last_action_error.unwrap_or(""))
        .size(12)
        .color(OFF_WHITE.scale_alpha(0.8));
    let explorer = container(column![explorer_title, explorer_error, explorer_canvas].spacing(5));

    let right = column![explorer].spacing(20).width(Length::FillPortion(2));

    row![left, right].spacing(20).into()
}
