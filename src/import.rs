//! Helps to import bank files.

use std::io::Read;

use crate::units::{Amount, Date};

#[derive(Debug, Clone)]
/// Used to describe a bank account movment (in or out)
pub struct BankLine {
    /// Date of operation
    date_op: Date,
    /// Date of validation
    date_val: Date,
    /// Label of operation
    label: String,
    /// Category from parent to son
    category: Vec<String>,
    /// From / to
    supplier_found: String,
    /// Amount (positive = gain)
    amount: Amount,
    /// Comments on operation
    comment: String,
    /// Account number
    account_number: u32,
    /// Name of the account
    account_label: String,
    /// Balance after operation
    account_balance: Amount,
}

/// Parses a csv when given a separator, index of column of each field, and parse functions.
/// 
/// Returns a vec of the valid lines
fn from_csv<R: Read, C: AsRef<[usize]>, D, A>(
    reader: R,
    sep: u8,
    date_op_idx: usize,
    date_val_idx: usize,
    label_idx: usize,
    category_idx: C, // From parent to son.
    supplyer_found_idx: usize,
    amount_idx: usize,
    comment_idx: usize,
    account_number_idx: usize,
    account_label_idx: usize,
    account_balance_idx: usize,
    date_parser: D,
    amount_parser: A,
) -> Vec<BankLine>
where
    D: Fn(&str) -> Option<Date>,
    A: Fn(&str) -> Option<Amount>,
{
    let mut rdr = csv::ReaderBuilder::new().delimiter(sep).from_reader(reader);

    rdr.records()
        .filter_map(Result::ok) // Remove bad lines
        .map(|line| {
            let vec: Vec<_> = line.iter().collect();
            let date_op = date_parser(vec.get(date_op_idx)?)?;
            let date_val = date_parser(vec.get(date_val_idx)?)?;
            let label = vec.get(label_idx)?.to_string();
            let category = category_idx
                .as_ref()
                .iter()
                .map(|idx| Some(vec.get(*idx)?.to_string()))
                .collect::<Option<Vec<_>>>()?;
            let supplier_found = vec.get(supplyer_found_idx)?.to_string();
            let amount = amount_parser(vec.get(amount_idx)?)?;
            let comment = vec.get(comment_idx)?.to_string();
            let account_number = vec.get(account_number_idx)?.parse().ok()?;
            let account_label = vec.get(account_label_idx)?.to_string();
            let account_balance = amount_parser(vec.get(account_balance_idx)?)?;

            Some(BankLine {
                date_op,
                date_val,
                label,
                category,
                supplier_found,
                amount,
                comment,
                account_number,
                account_label,
                account_balance,
            })
        })
        .filter_map(|x| x)
        .collect()
}


/// Reads a csv from Boursobank.
/// 
/// Returns a [Vec] of all the lines contained in the file.
pub fn from_boursobank_csv<R: Read>(reader: R) -> Vec<BankLine> {
    from_csv(
        reader,
        b';',
        0,
        1,
        2,
        [3, 4],
        5,
        6,
        7,
        8,
        9,
        10,
        |s| Date::from_yyyy_mm_dd(s, '-'),
        |s| Amount::parse_euro(s)
    )
}
