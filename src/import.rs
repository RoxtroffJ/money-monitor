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
    sender_reciever: String,
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
    pub fn new<S1, S2, S3, S4, S5>(
        date_op: Date,
        date_val: Date,
        label: S1,
        category: Vec<S2>,
        supplier_found: S3,
        amount: Amount,
        comment: S4,
        account_number: u32,
        account_label: S5,
        account_balance: Amount,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>,
    {
        Self {
            date_op,
            date_val,
            label: label.into(),
            category: category.into_iter().map(|c| c.into()).collect(),
            sender_reciever: supplier_found.into(),
            amount,
            comment: comment.into(),
            account_number,
            account_label: account_label.into(),
            account_balance,
        }
    }

    /// Gets the date of the operation.
    pub fn get_date_op(&self) -> &Date {
        &self.date_op
    }
    /// Gets the date of validation.
    pub fn get_date_val(&self) -> &Date {
        &self.date_val
    }
    /// Gets the label.
    pub fn get_label(&self) -> &String {
        &self.label
    }
    /// Gets the category.
    pub fn get_category(&self) -> &Vec<String> {
        &self.category
    }
    /// Gets the sender/reciever.
    pub fn get_sender_reciever(&self) -> &String {
        &self.sender_reciever
    }
    /// Gets the amount of the operation (positive means gain).
    pub fn get_amount(&self) -> Amount {
        self.amount
    }
    /// Gets the comment on the operation.
    pub fn get_comment(&self) -> &String {
        &self.comment
    }
    /// Gets the account number.
    pub fn get_account_number(&self) -> u32 {
        self.account_number
    }
    /// Gets the account label.
    pub fn get_account_label(&self) -> &String {
        &self.account_label
    }
    /// Gets the account balance at day of operation.
    pub fn get_account_balance(&self) -> Amount {
        self.account_balance
    }
}

/// Parses a csv when given a separator, index of column of each field, and parse functions.
///
/// Returns an iterator over the valid lines
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
) -> impl Iterator<Item = BankLine>
where
    D: Fn(&str) -> Option<Date>,
    A: Fn(&str) -> Option<Amount>,
{
    let rdr = csv::ReaderBuilder::new().delimiter(sep).from_reader(reader);

    rdr.into_records()
        .filter_map(Result::ok) // Remove bad lines
        .map(move |line| {
            let vec: Vec<_> = line.iter().collect();
            let date_op = date_parser(vec.get(date_op_idx)?)?;
            let date_val = date_parser(vec.get(date_val_idx)?)?;
            let label = *vec.get(label_idx)?;
            let category = category_idx
                .as_ref()
                .iter()
                .map(|idx| Some(*vec.get(*idx)?))
                .collect::<Option<_>>()?;
            let supplier_found = *vec.get(supplyer_found_idx)?;
            let amount = amount_parser(vec.get(amount_idx)?)?;
            let comment = *vec.get(comment_idx)?;
            let account_number = vec.get(account_number_idx)?.parse().ok()?;
            let account_label = *vec.get(account_label_idx)?;
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
}

/// Reads a csv from Boursobank.
///
/// Returns an [Iterator] of all the lines contained in the file.
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
/// let mut iter = from_boursobank_csv(csv);
///
/// let line1 = BankLine::new(
///     Date::new(26, Month::August, 2025).unwrap(),
///     Date::new(26, Month::August, 2025).unwrap(),
///     "FOO1",
///     vec!["Bâr", "Bâr"],
///     "BAZ",
///     Amount::new(-101.00),
///     "",
///     42,
///     "BoursoBank",
///     Amount::new(1057.24)
/// );
/// let line2 = BankLine::new(
///     Date::new(26, Month::August, 2025).unwrap(),
///     Date::new(26, Month::August, 2025).unwrap(),
///     "FOO2",
///     vec!["Bäär", "Bär"],
///     "baz",
///     Amount::new(-12.50),
///     "",
///     42,
///     "BoursoBank",
///     Amount::new(1057.24)
/// );
///
/// assert_eq!(Some(line1), iter.next());
/// assert_eq!(Some(line2), iter.next());
/// assert!(iter.next().is_none());
/// ```
pub fn from_boursobank_csv<R: Read>(reader: R) -> impl Iterator<Item = BankLine> {
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
