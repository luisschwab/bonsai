use bitcoin::Amount;
use bitcoin::Block;
use iced::Element;
use iced::Length;
use iced::Padding;
use iced::alignment::Horizontal::Left;
use iced::alignment::Horizontal::Right;
use iced::widget::Container;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::qr_code;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::scrollable::Scrollable;
use iced::widget::scrollable::Scrollbar;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::tooltip;
use tracing::info;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::container::common::CELL_HEIGHT;
use crate::common::interface::container::common::CELL_HEIGHT_2X;
use crate::common::interface::container::common::SHADOW;
use crate::common::interface::container::common::shadow_container;
use crate::common::interface::container::content::button_container;
use crate::common::interface::font::BERKELEY_MONO_BOLD;
use crate::common::util::format_thousands;
use crate::node::interface::common::TITLE_PADDING;
use crate::node::interface::common::input_field;
use crate::node::interface::common::table_cell;
use crate::node::interface::common::title_container;
use crate::node::interface::common::transparent_button;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;

const CELL_HEIGHT_PX: f32 = 35.0;

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

pub fn view_blocks<'a>(
    statistics: &'a Option<NodeStatistics>,
    block_height: &'a str,
    latest_blocks: &'a [Block],
    current_block: &'a Option<Block>,
    expanded_tx_idx: &'a Option<usize>,
) -> Element<'a, NodeMessage> {
    // Tab Title.
    let title: Container<'_, NodeMessage> = container(text("NODE BLOCKS").size(25))
        .style(title_container())
        .padding(TITLE_PADDING);

    let latest_title: Container<'_, NodeMessage> = container(text("LATEST BLOCKS").size(24));
    let latest_canvas: Container<'_, NodeMessage> = {
        let blocks_column = latest_blocks.iter().take(5).enumerate().fold(
            column![].spacing(0),
            |col, (idx, block)| {
                let block_height = block.bip34_block_height().unwrap_or(0);
                let tx_count = block.txdata.len();
                let block_size_bytes = bitcoin::consensus::encode::serialize(&block).len();
                let block_size = if block_size_bytes < 1_000 {
                    format!("{} BYTES", block_size_bytes)
                } else if block_size_bytes < 1_000_000 {
                    format!("{:.2} KB", block_size_bytes as f64 / 1_000.0)
                } else {
                    format!("{:.2} MB", block_size_bytes as f64 / 1_000_000.0)
                };

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
                .align_y(iced::alignment::Vertical::Center);

                let block_button = button(container(block_info).padding(10))
                    .style(transparent_button())
                    .on_press(NodeMessage::BlockExplorerHeightUpdate(block_height));

                col.push(block_button)
            },
        );

        container(blocks_column)
            .padding(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(iced::alignment::Vertical::Center)
            .style(title_container())
    };

    let latest = container(column![latest_title, latest_canvas].spacing(5));

    let left = column![title, latest]
        .spacing(20)
        .width(Length::FillPortion(1));

    // Parse the `block_height` string into a `u32`.
    let current_height = block_height.replace(",", "").parse::<u32>().ok();
    let explorer_title = container(row![
        text("BLOCK EXPLORER").size(24),
        Space::new().width(Length::Fill),
        row![
            button(
                text("<")
                    .size(16)
                    .font(BERKELEY_MONO_BOLD)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
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
                    .align_x(iced::alignment::Horizontal::Center)
                    .width(Length::Fixed(110.0))
            )
            .padding(0)
            .style(shadow_container())
            .height(CELL_HEIGHT),
            button(
                text(">")
                    .size(16)
                    .font(BERKELEY_MONO_BOLD)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
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
    ]);

    let header_table = {
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
            let hex = b.header.prev_blockhash.to_string();
            format!("{}\n{}", &hex[..32], &hex[32..])
        });
        let merkle_root = current_block.as_ref().map_or(String::new(), |b| {
            let hex = b.header.merkle_root.to_string();
            format!("{}\n{}", &hex[..32], &hex[32..])
        });

        let (block_size, block_weight, subsidy_and_fees, total_moved) =
            current_block.as_ref().map_or(
                (String::new(), String::new(), String::new(), String::new()),
                |block| {
                    let block_size_bytes = bitcoin::consensus::encode::serialize(&block).len();
                    let block_size = if block_size_bytes < 1_000 {
                        format!("{} BYTES", block_size_bytes)
                    } else if block_size_bytes < 1_000_000 {
                        format!("{:.2} KB", block_size_bytes as f64 / 1_000.0)
                    } else {
                        format!("{:.2} MB", block_size_bytes as f64 / 1_000_000.0)
                    };

                    let block_weight =
                        format!("{} WU", format_thousands(block.weight().to_wu() as u32));

                    // Need to fetch all prevouts for fees (too network intensive?)
                    let fees = Amount::from_sat(0);
                    let subsidy = Amount::from_sat(get_block_subsidy(current_height.unwrap_or(0)));
                    let subsidy_and_fees =
                        format!("{} BTC", format_thousands((subsidy + fees).to_btc()));

                    let mut total_moved = Amount::from_sat(0);
                    for tx in &block.txdata {
                        let output_sum: u64 = tx.output.iter().map(|o| o.value.to_sat()).sum();
                        total_moved += Amount::from_sat(output_sum);
                    }

                    let total_moved = format!(
                        "{} BTC",
                        format_thousands(format!("{:.2}", total_moved.to_btc()))
                    );

                    (block_size, block_weight, subsidy_and_fees, total_moved)
                },
            );

        column![
            row![
                container(text("HEADER & STATS").font(BERKELEY_MONO_BOLD).size(16))
                    .width(Length::Fill)
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
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
                    .align_y(iced::alignment::Vertical::Center)
                    .style(table_cell()),
            ]
            .spacing(0),
        ]
        .spacing(0)
    };

    let mut transactions_table = column![
        row![
            container(text("TRANSACTIONS").font(BERKELEY_MONO_BOLD).size(16))
                .width(Length::Fill)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .height(CELL_HEIGHT)
                .style(table_cell()),
        ]
        .spacing(0),
        row![
            container(text("IDX").font(BERKELEY_MONO_BOLD).size(14))
                .width(Length::Fixed(80.0))
                .height(CELL_HEIGHT)
                .padding(0)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .style(table_cell()),
            container(text("TXID").font(BERKELEY_MONO_BOLD).size(14))
                .width(Length::Fill)
                .height(CELL_HEIGHT)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .style(table_cell()),
        ]
        .spacing(0),
    ]
    .spacing(0);

    if let Some(block) = current_block {
        for (idx, tx) in block.txdata.iter().enumerate() {
            let txid = tx.compute_txid().to_string();

            let is_expanded = *expanded_tx_idx == Some(idx);

            let tx_row = button(
                row![
                    container(text(format!("{:05}", idx)).size(12))
                        .width(Length::Fixed(80.0))
                        .height(CELL_HEIGHT)
                        .align_y(iced::alignment::Vertical::Center)
                        .align_x(iced::alignment::Horizontal::Center)
                        .style(table_cell()),
                    container(text(txid).size(12))
                        .width(Length::Fill)
                        .height(CELL_HEIGHT)
                        .align_y(iced::alignment::Vertical::Center)
                        .align_x(iced::alignment::Horizontal::Center)
                        .style(table_cell()),
                ]
                .spacing(0),
            )
            .on_press(NodeMessage::ToggleTransactionExpandedIdx(idx))
            .style(button_container())
            .padding(0);

            transactions_table = transactions_table.push(tx_row);

            if is_expanded {
                let tx_details = {
                    let mut details = column![
                        row![
                            container(text("VERSION").font(BERKELEY_MONO_BOLD).size(12))
                                .width(Length::FillPortion(1))
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .style(table_cell()),
                            container(text(format!("{:08x}", tx.version.0)).size(12))
                                .width(Length::FillPortion(1))
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .align_y(iced::alignment::Vertical::Center)
                                .style(table_cell()),
                            container(text("INPUT COUNT").font(BERKELEY_MONO_BOLD).size(12))
                                .width(Length::FillPortion(1))
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .style(table_cell()),
                            container(text(format!("{:04}", tx.input.len())).size(12))
                                .width(Length::FillPortion(1))
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .align_y(iced::alignment::Vertical::Center)
                                .style(table_cell()),
                        ]
                        .spacing(0),
                        row![
                            container(text("LOCKTIME").font(BERKELEY_MONO_BOLD).size(12))
                                .width(Length::FillPortion(1))
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .style(table_cell()),
                            container(
                                text(if tx.lock_time.is_block_height() {
                                    format!("BLOCKS: {}", tx.lock_time.to_consensus_u32())
                                } else {
                                    format!("SECONDS: {}", tx.lock_time.to_consensus_u32())
                                })
                                .size(12)
                            )
                            .width(Length::FillPortion(1))
                            .height(CELL_HEIGHT)
                            .padding(10)
                            .align_y(iced::alignment::Vertical::Center)
                            .style(table_cell()),
                            container(text("OUTPUT COUNT").font(BERKELEY_MONO_BOLD).size(12))
                                .width(Length::FillPortion(1))
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .style(table_cell()),
                            container(text(format!("{:04}", tx.output.len())).size(12))
                                .width(Length::FillPortion(1))
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .align_y(iced::alignment::Vertical::Center)
                                .style(table_cell()),
                        ]
                        .spacing(0),
                        row![
                            container(text("INPUTS").font(BERKELEY_MONO_BOLD).size(14))
                                .width(Length::Fill)
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .style(table_cell()),
                        ]
                        .spacing(0),
                    ]
                    .spacing(0);

                    for (input_idx, input) in tx.input.iter().enumerate() {
                        let prevout_txid = input.previous_output.txid.to_string();
                        let prevout_vout = input.previous_output.vout.to_string();
                        let prevout = format!("{}:{}", prevout_txid, prevout_vout);

                        let sequence = format!("{:08x}", input.sequence);
                        let script_sig = input.script_sig.to_asm_string();

                        let witness_count = input.witness.len();

                        let mut input_rows = column![
                            row![
                                container(text("OUTPOINT").size(12))
                                    .width(Length::FillPortion(1))
                                    .padding(10)
                                    .align_y(iced::alignment::Vertical::Center)
                                    .align_x(iced::alignment::Horizontal::Center)
                                    .style(table_cell()),
                                container(text(prevout).size(12).wrapping(text::Wrapping::Glyph))
                                    .width(Length::FillPortion(3))
                                    .padding(10)
                                    .align_y(iced::alignment::Vertical::Center)
                                    .style(table_cell()),
                            ]
                            .spacing(0),
                            row![
                                container(text("SEQUENCE").size(12))
                                    .width(Length::FillPortion(1))
                                    .padding(10)
                                    .align_y(iced::alignment::Vertical::Center)
                                    .align_x(iced::alignment::Horizontal::Center)
                                    .style(table_cell()),
                                container(text(sequence).size(12))
                                    .width(Length::FillPortion(3))
                                    .padding(10)
                                    .align_y(iced::alignment::Vertical::Center)
                                    .style(table_cell()),
                            ]
                            .spacing(0),
                            row![
                                container(text("SCRIPTSIG").size(12))
                                    .width(Length::FillPortion(1))
                                    .padding(10)
                                    .align_y(iced::alignment::Vertical::Center)
                                    .align_x(iced::alignment::Horizontal::Center)
                                    .style(table_cell()),
                                container(
                                    text(if script_sig.is_empty() {
                                        String::from("EMPTY")
                                    } else {
                                        script_sig
                                    })
                                    .size(12)
                                    .wrapping(text::Wrapping::Glyph)
                                )
                                .width(Length::FillPortion(3))
                                .padding(12)
                                .align_y(iced::alignment::Vertical::Center)
                                .style(table_cell()),
                            ]
                            .spacing(0),
                        ]
                        .spacing(0);

                        if witness_count == 0 {
                            input_rows = input_rows.push(
                                row![
                                    container(text("WITNESS").size(12))
                                        .width(Length::FillPortion(1))
                                        .padding(10)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .style(table_cell()),
                                    container(
                                        text("EMPTY").size(12).color(OFF_WHITE.scale_alpha(0.5))
                                    )
                                    .width(Length::FillPortion(3))
                                    .padding(10)
                                    .align_y(iced::alignment::Vertical::Center)
                                    .style(table_cell()),
                                ]
                                .spacing(0),
                            );
                        } else {
                            for (witness_idx, witness_item) in input.witness.iter().enumerate() {
                                let witness_hex = hex::encode(witness_item);
                                input_rows = input_rows.push(
                                    row![
                                        container(
                                            text(format!("WITNESS {}", witness_idx)).size(12)
                                        )
                                        .width(Length::FillPortion(1))
                                        .padding(10)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .style(table_cell()),
                                        container(
                                            text(witness_hex)
                                                .size(12)
                                                .wrapping(text::Wrapping::Glyph)
                                        )
                                        .width(Length::FillPortion(3))
                                        .padding(10)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .style(table_cell()),
                                    ]
                                    .spacing(0),
                                );
                            }
                        }

                        details = details.push(
                            row![
                                container(
                                    text(format!("{:02}", input_idx))
                                        .font(BERKELEY_MONO_BOLD)
                                        .size(20)
                                )
                                .width(Length::Fixed(80.0))
                                .height(Length::Fill) // Changed from Shrink to Fill
                                .padding(10)
                                .align_y(iced::alignment::Vertical::Center)
                                .align_x(iced::alignment::Horizontal::Center)
                                .style(table_cell()),
                                input_rows.width(Length::Fill),
                            ]
                            .spacing(0),
                        );
                    }

                    details = details.push(
                        row![
                            container(text("OUTPUTS").font(BERKELEY_MONO_BOLD).size(14))
                                .width(Length::Fill)
                                .height(CELL_HEIGHT)
                                .padding(10)
                                .align_x(iced::alignment::Horizontal::Center)
                                .align_y(iced::alignment::Vertical::Center)
                                .style(table_cell()),
                        ]
                        .spacing(0),
                    );

                    for (output_idx, output) in tx.output.iter().enumerate() {
                        let value = format!("{} SATOSHIS", format_thousands(output.value.to_sat()));
                        let script_pubkey = output.script_pubkey.to_asm_string();
                        let script_type = if output.script_pubkey.is_p2pkh() {
                            "P2PKH"
                        } else if output.script_pubkey.is_p2sh() {
                            "P2SH"
                        } else if output.script_pubkey.is_p2wpkh() {
                            "P2WPKH"
                        } else if output.script_pubkey.is_p2wsh() {
                            "P2WSH"
                        } else if output.script_pubkey.is_p2tr() {
                            "P2TR"
                        } else if output.script_pubkey.is_op_return() {
                            "OP_RETURN"
                        } else {
                            "UNKNOWN"
                        };

                        details = details.push(
                            row![
                                container(
                                    text(format!("{:02}", output_idx))
                                        .font(BERKELEY_MONO_BOLD)
                                        .size(20)
                                )
                                .width(Length::Fixed(80.0))
                                .height(Length::Fill)
                                .padding(10)
                                .align_y(iced::alignment::Vertical::Center)
                                .align_x(iced::alignment::Horizontal::Center)
                                .style(table_cell()),
                                column![
                                    row![
                                        container(text("VALUE").size(12))
                                            .width(Length::FillPortion(1))
                                            .padding(10)
                                            .align_y(iced::alignment::Vertical::Center)
                                            .align_x(iced::alignment::Horizontal::Center)
                                            .style(table_cell()),
                                        container(text(value).size(12))
                                            .width(Length::FillPortion(3))
                                            .padding(10)
                                            .align_y(iced::alignment::Vertical::Center)
                                            .style(table_cell()),
                                    ]
                                    .spacing(0),
                                    row![
                                        container(text("SCRIPT TYPE").size(12))
                                            .width(Length::FillPortion(1))
                                            .padding(10)
                                            .align_y(iced::alignment::Vertical::Center)
                                            .align_x(iced::alignment::Horizontal::Center)
                                            .style(table_cell()),
                                        container(text(script_type).size(12))
                                            .width(Length::FillPortion(3))
                                            .padding(10)
                                            .align_y(iced::alignment::Vertical::Center)
                                            .style(table_cell()),
                                    ]
                                    .spacing(0),
                                    row![
                                        container(text("SCRIPTPUBKEY").size(12))
                                            .width(Length::FillPortion(1))
                                            .padding(10)
                                            .align_y(iced::alignment::Vertical::Center)
                                            .align_x(iced::alignment::Horizontal::Center)
                                            .style(table_cell()),
                                        container(
                                            text(script_pubkey)
                                                .size(12)
                                                .wrapping(text::Wrapping::Glyph)
                                        )
                                        .width(Length::FillPortion(3))
                                        .padding(10)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .style(table_cell()),
                                    ]
                                    .spacing(0),
                                ]
                                .spacing(0)
                                .width(Length::Fill),
                            ]
                            .spacing(0),
                        );
                    }

                    container(details)
                        .width(Length::Fill)
                        .padding(Padding::from([0, 30]))
                        .style(table_cell())
                };

                transactions_table = transactions_table.push(tx_details);
            }
        }
    }

    let transactions_scrollable = scrollable(transactions_table)
        .height(Length::Fill)
        .direction(iced::widget::scrollable::Direction::Vertical(
            Scrollbar::hidden(),
        ));

    let explorer_canvas = container(column![header_table, transactions_scrollable,].spacing(0))
        .padding(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(title_container());

    let explorer = container(column![explorer_title, explorer_canvas].spacing(5));

    let right = column![explorer].spacing(20).width(Length::FillPortion(2));

    row![left, right].spacing(20).into()
}
