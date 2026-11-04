use super::*;

#[test]
fn preserves_each_amount_without_summing_or_narrowing() {
    let total = Int::from(10);
    let prepaid = Int::from(u128::MAX);
    let promotional = Int::from(0);
    let ledger = Int::from(3);
    let amounts = balance_amounts_from_candid(BalanceAmountsInput {
        total: &total,
        prepaid: &prepaid,
        promotional: &promotional,
        ledger: &ledger,
    })
    .expect("independently bounded amounts");
    assert_eq!(amounts.total(), 10);
    assert_eq!(amounts.prepaid(), u128::MAX);
    assert_eq!(amounts.promotional(), 0);
    assert_eq!(amounts.ledger(), 3);
}

#[test]
fn every_malformed_component_rejects_the_whole_balance() {
    let valid = Int::from(u128::MAX);
    for invalid in [
        Int::from(-1),
        Int::parse(b"340282366920938463463374607431768211456").expect("oversized integer"),
    ] {
        for (index, field) in [
            BalanceField::Total,
            BalanceField::Prepaid,
            BalanceField::Promotional,
            BalanceField::Ledger,
        ]
        .into_iter()
        .enumerate()
        {
            let mut values = [&valid; 4];
            values[index] = &invalid;
            assert_eq!(
                balance_amounts_from_candid(BalanceAmountsInput {
                    total: values[0],
                    prepaid: values[1],
                    promotional: values[2],
                    ledger: values[3],
                }),
                Err(BalanceInputError { field }),
            );
        }
    }
}
