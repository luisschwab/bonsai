use bitcoin::Block;
use bitcoin::Transaction;
use iced::Alignment::Center;
use iced::Length;
use iced::Padding;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::text;

use crate::common::interface::color::OFF_WHITE;
use crate::common::interface::constants::CELL_HEIGHT;
use crate::common::interface::container::button_container;
use crate::common::interface::font::BERKELEY_MONO_BOLD;
use crate::common::util::format_thousands;
use crate::node::message::NodeMessage;
use crate::node::style::table_cell;

fn label_cell<'a>(label: impl Into<String>) -> Container<'a, NodeMessage> {
    container(text(label.into()).size(12))
        .width(Length::FillPortion(1))
        .padding(10)
        .align_y(Center)
        .align_x(Center)
        .style(table_cell())
}

fn value_cell<'a>(value: impl Into<String>) -> Container<'a, NodeMessage> {
    container(text(value.into()).size(12))
        .width(Length::FillPortion(3))
        .padding(10)
        .align_y(Center)
        .style(table_cell())
}

fn wrapping_value_cell<'a>(value: impl Into<String>) -> Container<'a, NodeMessage> {
    container(text(value.into()).size(12).wrapping(text::Wrapping::Glyph))
        .width(Length::FillPortion(3))
        .padding(10)
        .align_y(Center)
        .style(table_cell())
}

fn empty_value_cell<'a>() -> Container<'a, NodeMessage> {
    container(text("EMPTY").size(12).color(OFF_WHITE.scale_alpha(0.5)))
        .width(Length::FillPortion(3))
        .padding(10)
        .align_y(Center)
        .style(table_cell())
}

fn section_header<'a>(label: &'static str) -> iced::widget::Row<'a, NodeMessage> {
    row![
        container(text(label).font(BERKELEY_MONO_BOLD).size(14))
            .width(Length::Fill)
            .height(CELL_HEIGHT)
            .padding(10)
            .align_x(Center)
            .align_y(Center)
            .style(table_cell()),
    ]
    .spacing(0)
}

fn index_cell<'a>(idx: usize) -> Container<'a, NodeMessage> {
    container(
        text(format!("{:02}", idx))
            .font(BERKELEY_MONO_BOLD)
            .size(20),
    )
    .width(Length::Fixed(80.0))
    .height(Length::Fill)
    .padding(10)
    .align_y(Center)
    .align_x(Center)
    .style(table_cell())
}

fn transaction_details<'a>(tx: &'a Transaction) -> Container<'a, NodeMessage> {
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
                .align_y(Center)
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
                .align_y(Center)
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
            .align_y(Center)
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
                .align_y(Center)
                .style(table_cell()),
        ]
        .spacing(0),
        section_header("INPUTS"),
    ]
    .spacing(0);

    for (input_idx, input) in tx.input.iter().enumerate() {
        let prevout_txid = input.previous_output.txid.to_string();
        let prevout_vout = input.previous_output.vout.to_string();
        let prevout = format!("{}:{}", prevout_txid, prevout_vout);

        let sequence = format!("{:08x}", input.sequence);
        let script_sig = input.script_sig.to_asm_string();

        let mut input_rows = column![
            row![label_cell("OUTPOINT"), wrapping_value_cell(prevout)].spacing(0),
            row![label_cell("SEQUENCE"), value_cell(sequence)].spacing(0),
            row![
                label_cell("SCRIPTSIG"),
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
                .align_y(Center)
                .style(table_cell()),
            ]
            .spacing(0),
        ]
        .spacing(0);

        if input.witness.is_empty() {
            input_rows =
                input_rows.push(row![label_cell("WITNESS"), empty_value_cell()].spacing(0));
        } else {
            for (witness_idx, witness_item) in input.witness.iter().enumerate() {
                let witness_hex = hex::encode(witness_item);
                input_rows = input_rows.push(
                    row![
                        label_cell(format!("WITNESS {}", witness_idx)),
                        wrapping_value_cell(witness_hex),
                    ]
                    .spacing(0),
                );
            }
        }

        details =
            details.push(row![index_cell(input_idx), input_rows.width(Length::Fill),].spacing(0));
    }

    details = details.push(section_header("OUTPUTS"));

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
                index_cell(output_idx),
                column![
                    row![label_cell("VALUE"), value_cell(value)].spacing(0),
                    row![label_cell("SCRIPT TYPE"), value_cell(script_type)].spacing(0),
                    row![
                        label_cell("SCRIPTPUBKEY"),
                        wrapping_value_cell(script_pubkey)
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
}

pub(crate) fn transactions_table<'a>(
    current_block: &'a Option<Block>,
    expanded_tx_idx: &'a Option<usize>,
) -> Column<'a, NodeMessage> {
    let mut transactions_table = column![
        row![
            container(text("TRANSACTIONS").font(BERKELEY_MONO_BOLD).size(16))
                .width(Length::Fill)
                .align_y(Center)
                .align_x(Center)
                .height(CELL_HEIGHT)
                .style(table_cell()),
        ]
        .spacing(0),
        row![
            container(text("IDX").font(BERKELEY_MONO_BOLD).size(14))
                .width(Length::Fixed(80.0))
                .height(CELL_HEIGHT)
                .padding(0)
                .align_y(Center)
                .align_x(Center)
                .style(table_cell()),
            container(text("TXID").font(BERKELEY_MONO_BOLD).size(14))
                .width(Length::Fill)
                .height(CELL_HEIGHT)
                .align_x(Center)
                .align_y(Center)
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
                        .align_y(Center)
                        .align_x(Center)
                        .style(table_cell()),
                    container(text(txid).size(12))
                        .width(Length::Fill)
                        .height(CELL_HEIGHT)
                        .align_y(Center)
                        .align_x(Center)
                        .style(table_cell()),
                ]
                .spacing(0),
            )
            .on_press(NodeMessage::ToggleTransactionExpandedIdx(idx))
            .style(button_container())
            .padding(0);

            transactions_table = transactions_table.push(tx_row);

            if is_expanded {
                transactions_table = transactions_table.push(transaction_details(tx));
            }
        }
    }

    transactions_table
}
