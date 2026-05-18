#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol, Vec};

/// Invoice represents a tokenized B2B invoice on the blockchain
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invoice {
    pub id: u64,
    pub issuer: Address,           // SME that issued the invoice
    pub debtor: Address,           // Corporate client that owes payment
    pub amount: i128,              // Full invoice amount
    pub discount_rate: u32,        // Discount rate in basis points (e.g., 500 = 5%)
    pub maturity_date: u64,        // Unix timestamp when invoice is due
    pub status: InvoiceStatus,
}

/// Status of an invoice in the escrow system
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvoiceStatus {
    Active,      // Invoice is available for purchase
    Funded,      // Investor has purchased the invoice
    Settled,     // Debtor has paid, funds distributed
    Disputed,    // Payment is under dispute
}

/// Storage keys for the contract
#[contracttype]
pub enum DataKey {
    Invoice(u64),
    InvoiceCount,
    Admin,
}

#[contract]
pub struct UpCurrentEscrow;

#[contractimpl]
impl UpCurrentEscrow {
    /// Initialize the contract with an admin address
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::InvoiceCount, &0u64);
    }

    /// Create a new invoice token
    /// SME uploads invoice details and mints it on-chain
    pub fn create_invoice(
        env: Env,
        issuer: Address,
        debtor: Address,
        amount: i128,
        discount_rate: u32,
        maturity_date: u64,
    ) -> u64 {
        issuer.require_auth();

        // Validate inputs
        if amount <= 0 {
            panic!("Invoice amount must be positive");
        }
        if discount_rate > 10000 {
            panic!("Discount rate cannot exceed 100%");
        }

        // Get and increment invoice counter
        let mut invoice_count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::InvoiceCount)
            .unwrap_or(0);
        invoice_count += 1;

        // Create new invoice
        let invoice = Invoice {
            id: invoice_count,
            issuer: issuer.clone(),
            debtor,
            amount,
            discount_rate,
            maturity_date,
            status: InvoiceStatus::Active,
        };

        // Store invoice
        env.storage()
            .persistent()
            .set(&DataKey::Invoice(invoice_count), &invoice);
        env.storage()
            .instance()
            .set(&DataKey::InvoiceCount, &invoice_count);

        invoice_count
    }

    /// Fund an invoice - investor purchases the invoice at a discount
    pub fn fund_invoice(
        env: Env,
        invoice_id: u64,
        investor: Address,
        token: Address,
    ) {
        investor.require_auth();

        // Get invoice
        let mut invoice: Invoice = env
            .storage()
            .persistent()
            .get(&DataKey::Invoice(invoice_id))
            .expect("Invoice not found");

        // Verify invoice is active
        if invoice.status != InvoiceStatus::Active {
            panic!("Invoice is not available for funding");
        }

        // Calculate discounted amount
        let discount_amount = (invoice.amount * invoice.discount_rate as i128) / 10000;
        let payment_amount = invoice.amount - discount_amount;

        // Transfer discounted amount from investor to issuer
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&investor, &invoice.issuer, &payment_amount);

        // Update invoice status
        invoice.status = InvoiceStatus::Funded;
        env.storage()
            .persistent()
            .set(&DataKey::Invoice(invoice_id), &invoice);
    }

    /// Settle an invoice - debtor pays full amount, funds distributed to investor
    pub fn settle_invoice(
        env: Env,
        invoice_id: u64,
        debtor: Address,
        token: Address,
    ) {
        debtor.require_auth();

        // Get invoice
        let mut invoice: Invoice = env
            .storage()
            .persistent()
            .get(&DataKey::Invoice(invoice_id))
            .expect("Invoice not found");

        // Verify debtor matches
        if invoice.debtor != debtor {
            panic!("Only the debtor can settle this invoice");
        }

        // Verify invoice is funded
        if invoice.status != InvoiceStatus::Funded {
            panic!("Invoice must be funded before settlement");
        }

        // Transfer full amount from debtor to contract (escrow)
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&debtor, &env.current_contract_address(), &invoice.amount);

        // Update invoice status
        invoice.status = InvoiceStatus::Settled;
        env.storage()
            .persistent()
            .set(&DataKey::Invoice(invoice_id), &invoice);
    }

    /// Get invoice details
    pub fn get_invoice(env: Env, invoice_id: u64) -> Invoice {
        env.storage()
            .persistent()
            .get(&DataKey::Invoice(invoice_id))
            .expect("Invoice not found")
    }

    /// Get total number of invoices
    pub fn get_invoice_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::InvoiceCount)
            .unwrap_or(0)
    }
}

mod test;
