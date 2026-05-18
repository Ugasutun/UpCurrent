#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UpCurrentEscrow);
    let client = UpCurrentEscrowClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Verify invoice count starts at 0
    assert_eq!(client.get_invoice_count(), 0);
}

#[test]
fn test_create_invoice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, UpCurrentEscrow);
    let client = UpCurrentEscrowClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let debtor = Address::generate(&env);

    client.initialize(&admin);

    // Create an invoice
    let invoice_id = client.create_invoice(
        &issuer,
        &debtor,
        &1000000, // $1000 in smallest units
        &500,     // 5% discount
        &1735689600, // Future maturity date
    );

    assert_eq!(invoice_id, 1);
    assert_eq!(client.get_invoice_count(), 1);

    // Verify invoice details
    let invoice = client.get_invoice(&invoice_id);
    assert_eq!(invoice.issuer, issuer);
    assert_eq!(invoice.debtor, debtor);
    assert_eq!(invoice.amount, 1000000);
    assert_eq!(invoice.discount_rate, 500);
    assert_eq!(invoice.status, InvoiceStatus::Active);
}

#[test]
#[should_panic(expected = "Invoice amount must be positive")]
fn test_create_invoice_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, UpCurrentEscrow);
    let client = UpCurrentEscrowClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let debtor = Address::generate(&env);

    client.initialize(&admin);

    // Try to create invoice with negative amount
    client.create_invoice(&issuer, &debtor, &-1000, &500, &1735689600);
}

#[test]
#[should_panic(expected = "Discount rate cannot exceed 100%")]
fn test_create_invoice_invalid_discount() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, UpCurrentEscrow);
    let client = UpCurrentEscrowClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let debtor = Address::generate(&env);

    client.initialize(&admin);

    // Try to create invoice with invalid discount rate
    client.create_invoice(&issuer, &debtor, &1000000, &15000, &1735689600);
}
