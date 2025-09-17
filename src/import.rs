//! Helps to import bank files.

use std::io::Read;

use crate::units::{Amount, Date};

#[derive(Debug, Clone, PartialEq)]
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

impl BankLine {
    /// Creates a new bank line.
    pub fn new(
        date_op: Date,
        date_val: Date,
        label: String,
        category: Vec<String>,
        supplier_found: String,
        amount: Amount,
        comment: String,
        account_number: u32,
        account_label: String,
        account_balance: Amount,
    ) -> Self {
        Self {
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
        }
    }
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

            Some(BankLine::new(
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
            ))
        })
        .filter_map(|x| x)
        .collect()
}

/// Reads a csv from Boursobank.
///
/// Returns a [Vec] of all the lines contained in the file.
///
/// # Example
/// ```
/// use std::io::Cursor;
/// use money_monitor::units::{Date, Month, Amount};
/// use money_monitor::import::{from_boursobank_csv, BankLine};
///
/// let csv = Cursor::new(
/// "dateOp;dateVal;label;category;categoryParent;supplierFound;amount;comment;accountNum;accountLabel;accountbalance
/// 2025-08-26;2025-08-26;\"FOO1\";\"Bâr\";\"Bâr\";\"BAZ\";-101,00;;42;BoursoBank;1057.24
/// 2025-08-26;2025-08-26;\"FOO2\";\"Bär\";\"Bäär\";\"baz\";-12,50;;42;BoursoBank;1057.24");
///
/// let vec = from_boursobank_csv(csv);
///
/// let line1 = BankLine::new(
///     Date::new(26, Month::August, 2025).unwrap(),
///     Date::new(26, Month::August, 2025).unwrap(),
///     "FOO1".to_string(),
///     vec!["Bâr".to_string(), "Bâr".to_string()],
///     "BAZ".to_string(),
///     Amount::new(-101.00),
///     "".to_string(),
///     42,
///     "BoursoBank".to_string(),
///     Amount::new(1057.24)
/// );
/// let line2 = BankLine::new(
///     Date::new(26, Month::August, 2025).unwrap(),
///     Date::new(26, Month::August, 2025).unwrap(),
///     "FOO2".to_string(),
///     vec!["Bäär".to_string(), "Bär".to_string()],
///     "baz".to_string(),
///     Amount::new(-12.50),
///     "".to_string(),
///     42,
///     "BoursoBank".to_string(),
///     Amount::new(1057.24)
/// );
/// 
/// assert_eq!(line1, vec[0]);
/// assert_eq!(line2, vec[1]);
/// ```
pub fn from_boursobank_csv<R: Read>(reader: R) -> Vec<BankLine> {
    from_csv(
        reader,
        b';',
        0,
        1,
        2,
        [4, 3],
        5,
        6,
        7,
        8,
        9,
        10,
        |s| Date::from_yyyy_mm_dd(s, '-'),
        |s| Amount::parse_euro(s.replace(',', ".")),
    )
}
