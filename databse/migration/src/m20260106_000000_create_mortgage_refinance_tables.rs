use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create borrowers table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("borrowers"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alias::new("name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("employer_name")).string().null())
                    .col(ColumnDef::new(Alias::new("income_type")).string().null())
                    .col(ColumnDef::new(Alias::new("loan_number")).string().null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create consumer_debts table
        manager
            .create_table(
                Table::create()
                    .table(ConsumerDebts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConsumerDebts::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ConsumerDebts::DebtorName).string().not_null())
                    .col(ColumnDef::new(ConsumerDebts::CreditType).string().not_null())
                    .col(ColumnDef::new(ConsumerDebts::Balance).decimal().not_null())
                    .col(ColumnDef::new(ConsumerDebts::MonthlyPayment).decimal().not_null())
                    .col(ColumnDef::new(ConsumerDebts::TermMonths).integer().null())
                    .col(ColumnDef::new(ConsumerDebts::InterestRate).decimal().null())
                    .col(ColumnDef::new(ConsumerDebts::OmitFromDti).boolean().not_null().default(false))
                    .col(ColumnDef::new(ConsumerDebts::PayOffAtClosing).boolean().not_null().default(false))
                    .col(ColumnDef::new(ConsumerDebts::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ConsumerDebts::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create existing_loans table
        manager
            .create_table(
                Table::create()
                    .table(ExistingLoans::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExistingLoans::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ExistingLoans::Position).small_integer().not_null())
                    .col(ColumnDef::new(ExistingLoans::LoanBalance).decimal().not_null())
                    .col(ColumnDef::new(ExistingLoans::MonthlyPayment).decimal().not_null())
                    .col(ColumnDef::new(ExistingLoans::RemainingTermMonths).integer().not_null())
                    .col(ColumnDef::new(ExistingLoans::InterestRate).decimal().not_null())
                    .col(ColumnDef::new(ExistingLoans::IsSubordinate).boolean().not_null().default(false))
                    .col(ColumnDef::new(ExistingLoans::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ExistingLoans::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create new_loan_details table
        manager
            .create_table(
                Table::create()
                    .table(NewLoanDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NewLoanDetails::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NewLoanDetails::MarketValue).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::SalesPrice).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::DownPayment).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::BaseLoanAmount).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::SubordinatedAmount).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::FfUmipPercentage).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::UmipRefund).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::TotalLoanAmount).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::NoteRate).decimal().not_null())
                    .col(ColumnDef::new(NewLoanDetails::AppraisalWaiver).boolean().not_null().default(false))
                    .col(ColumnDef::new(NewLoanDetails::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(NewLoanDetails::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create loan_information table
        manager
            .create_table(
                Table::create()
                    .table(LoanInformation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoanInformation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LoanInformation::PropertyType).string().not_null())
                    .col(ColumnDef::new(LoanInformation::OccupancyType).string().not_null())
                    .col(ColumnDef::new(LoanInformation::LoanType).string().not_null())
                    .col(ColumnDef::new(LoanInformation::NewTermMonths).integer().not_null())
                    .col(ColumnDef::new(LoanInformation::LoanPurpose).string().not_null())
                    .col(ColumnDef::new(LoanInformation::AppraisalWaiver).boolean().not_null().default(false))
                    .col(ColumnDef::new(LoanInformation::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(LoanInformation::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create benefit_to_borrower table
        manager
            .create_table(
                Table::create()
                    .table(BenefitToBorrower::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BenefitToBorrower::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BenefitToBorrower::ExistingPi).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ExistingTaxes).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ExistingInsurance).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ExistingFloodInsurance).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ExistingPmi).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ExistingHoa).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ExistingMortgagePayment).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedPi).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedTaxes).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedInsurance).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedFloodInsurance).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedPmi).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedHoa).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedMortgagePayment).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::EscrowTaxes).boolean().not_null().default(false))
                    .col(ColumnDef::new(BenefitToBorrower::EscrowInsurance).boolean().not_null().default(false))
                    .col(ColumnDef::new(BenefitToBorrower::EscrowFloodInsurance).boolean().not_null().default(false))
                    .col(ColumnDef::new(BenefitToBorrower::OverageShortage).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::DebtPaydown).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ExistingTotalObligations).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::ProposedTotalObligations).decimal().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(BenefitToBorrower::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create other_fees table
        manager
            .create_table(
                Table::create()
                    .table(OtherFees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OtherFees::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OtherFees::ThirdPartyFees).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::AppraisalFee).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::InvestorFee).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::PaddedTaxes).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::PaddedInsurance).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::LenderCredit).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::AdminFees).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::TaxService).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::FloodCertification).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::TotalClosingCosts).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::CashOutAmount).decimal().not_null())
                    .col(ColumnDef::new(OtherFees::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(OtherFees::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create pricing_options table
        manager
            .create_table(
                Table::create()
                    .table(PricingOptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PricingOptions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PricingOptions::Description).string().not_null())
                    .col(ColumnDef::new(PricingOptions::NoteRate).decimal().not_null())
                    .col(ColumnDef::new(PricingOptions::YspPercentage).decimal().not_null())
                    .col(ColumnDef::new(PricingOptions::YspDollar).decimal().not_null())
                    .col(ColumnDef::new(PricingOptions::BdPercentage).decimal().not_null())
                    .col(ColumnDef::new(PricingOptions::BdDollar).decimal().not_null())
                    .col(ColumnDef::new(PricingOptions::IsSelected).boolean().not_null().default(false))
                    .col(ColumnDef::new(PricingOptions::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(PricingOptions::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create income_information table
        manager
            .create_table(
                Table::create()
                    .table(IncomeInformation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IncomeInformation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IncomeInformation::BorrowerMonthlyIncome).decimal().not_null())
                    .col(ColumnDef::new(IncomeInformation::CoborrowerMonthlyIncome).decimal().not_null())
                    .col(ColumnDef::new(IncomeInformation::FrontEndRatio).decimal().not_null())
                    .col(ColumnDef::new(IncomeInformation::BackEndRatio).decimal().not_null())
                    .col(ColumnDef::new(IncomeInformation::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(IncomeInformation::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create savings_calculations table
        manager
            .create_table(
                Table::create()
                    .table(SavingsCalculations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SavingsCalculations::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SavingsCalculations::MonthlySavings).decimal().not_null())
                    .col(ColumnDef::new(SavingsCalculations::AnnualSavings).decimal().not_null())
                    .col(ColumnDef::new(SavingsCalculations::DebtPaid).decimal().not_null())
                    .col(ColumnDef::new(SavingsCalculations::PaymentReduction).decimal().not_null())
                    .col(ColumnDef::new(SavingsCalculations::RecoupPeriodMonths).decimal().not_null())
                    .col(ColumnDef::new(SavingsCalculations::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(SavingsCalculations::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create mortgage_refinance_options table
        manager
            .create_table(
                Table::create()
                    .table(MortgageRefinanceOptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MortgageRefinanceOptions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MortgageRefinanceOptions::BorrowerId).uuid().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::LoanInformationId).uuid().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::NewLoanDetailsId).uuid().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::BenefitToBorrowerId).uuid().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::OtherFeesId).uuid().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::IncomeInformationId).uuid().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::SavingsCalculationId).uuid().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::Status).string().not_null().default("draft"))
                    .col(ColumnDef::new(MortgageRefinanceOptions::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(MortgageRefinanceOptions::SubmittedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        // Create junction tables for relationships
        // mortgage_refinance_existing_loans junction table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("mortgage_refinance_existing_loans"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("mortgage_refinance_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("existing_loan_id")).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(Alias::new("mortgage_refinance_id"))
                            .col(Alias::new("existing_loan_id"))
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mortgage_refinance_existing_loans_mortgage_refinance")
                            .from(Alias::new("mortgage_refinance_existing_loans"), Alias::new("mortgage_refinance_id"))
                            .to(MortgageRefinanceOptions::Table, MortgageRefinanceOptions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mortgage_refinance_existing_loans_existing_loan")
                            .from(Alias::new("mortgage_refinance_existing_loans"), Alias::new("existing_loan_id"))
                            .to(ExistingLoans::Table, ExistingLoans::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // mortgage_refinance_pricing_options junction table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("mortgage_refinance_pricing_options"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("mortgage_refinance_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("pricing_option_id")).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(Alias::new("mortgage_refinance_id"))
                            .col(Alias::new("pricing_option_id"))
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mortgage_refinance_pricing_options_mortgage_refinance")
                            .from(Alias::new("mortgage_refinance_pricing_options"), Alias::new("mortgage_refinance_id"))
                            .to(MortgageRefinanceOptions::Table, MortgageRefinanceOptions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mortgage_refinance_pricing_options_pricing_option")
                            .from(Alias::new("mortgage_refinance_pricing_options"), Alias::new("pricing_option_id"))
                            .to(PricingOptions::Table, PricingOptions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // mortgage_refinance_consumer_debts junction table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("mortgage_refinance_consumer_debts"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("mortgage_refinance_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("consumer_debt_id")).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(Alias::new("mortgage_refinance_id"))
                            .col(Alias::new("consumer_debt_id"))
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mortgage_refinance_consumer_debts_mortgage_refinance")
                            .from(Alias::new("mortgage_refinance_consumer_debts"), Alias::new("mortgage_refinance_id"))
                            .to(MortgageRefinanceOptions::Table, MortgageRefinanceOptions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mortgage_refinance_consumer_debts_consumer_debt")
                            .from(Alias::new("mortgage_refinance_consumer_debts"), Alias::new("consumer_debt_id"))
                            .to(ConsumerDebts::Table, ConsumerDebts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(Alias::new("mortgage_refinance_consumer_debts")).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Alias::new("mortgage_refinance_pricing_options")).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Alias::new("mortgage_refinance_existing_loans")).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MortgageRefinanceOptions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SavingsCalculations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IncomeInformation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PricingOptions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OtherFees::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BenefitToBorrower::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LoanInformation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(NewLoanDetails::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ExistingLoans::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ConsumerDebts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Alias::new("borrowers")).to_owned())
            .await?;

        Ok(())
    }
}

// Table identifiers for the migration
#[derive(Iden)]
enum ConsumerDebts {
    Table,
    Id,
    DebtorName,
    CreditType,
    Balance,
    MonthlyPayment,
    TermMonths,
    InterestRate,
    OmitFromDti,
    PayOffAtClosing,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum ExistingLoans {
    Table,
    Id,
    Position,
    LoanBalance,
    MonthlyPayment,
    RemainingTermMonths,
    InterestRate,
    IsSubordinate,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum NewLoanDetails {
    Table,
    Id,
    MarketValue,
    SalesPrice,
    DownPayment,
    BaseLoanAmount,
    SubordinatedAmount,
    FfUmipPercentage,
    UmipRefund,
    TotalLoanAmount,
    NoteRate,
    AppraisalWaiver,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum LoanInformation {
    Table,
    Id,
    PropertyType,
    OccupancyType,
    LoanType,
    NewTermMonths,
    LoanPurpose,
    AppraisalWaiver,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum BenefitToBorrower {
    Table,
    Id,
    ExistingPi,
    ExistingTaxes,
    ExistingInsurance,
    ExistingFloodInsurance,
    ExistingPmi,
    ExistingHoa,
    ExistingMortgagePayment,
    ProposedPi,
    ProposedTaxes,
    ProposedInsurance,
    ProposedFloodInsurance,
    ProposedPmi,
    ProposedHoa,
    ProposedMortgagePayment,
    EscrowTaxes,
    EscrowInsurance,
    EscrowFloodInsurance,
    OverageShortage,
    DebtPaydown,
    ExistingTotalObligations,
    ProposedTotalObligations,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum OtherFees {
    Table,
    Id,
    ThirdPartyFees,
    AppraisalFee,
    InvestorFee,
    PaddedTaxes,
    PaddedInsurance,
    LenderCredit,
    AdminFees,
    TaxService,
    FloodCertification,
    TotalClosingCosts,
    CashOutAmount,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PricingOptions {
    Table,
    Id,
    Description,
    NoteRate,
    YspPercentage,
    YspDollar,
    BdPercentage,
    BdDollar,
    IsSelected,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum IncomeInformation {
    Table,
    Id,
    BorrowerMonthlyIncome,
    CoborrowerMonthlyIncome,
    FrontEndRatio,
    BackEndRatio,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum SavingsCalculations {
    Table,
    Id,
    MonthlySavings,
    AnnualSavings,
    DebtPaid,
    PaymentReduction,
    RecoupPeriodMonths,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum MortgageRefinanceOptions {
    Table,
    Id,
    BorrowerId,
    LoanInformationId,
    NewLoanDetailsId,
    BenefitToBorrowerId,
    OtherFeesId,
    IncomeInformationId,
    SavingsCalculationId,
    Status,
    CreatedAt,
    UpdatedAt,
    SubmittedAt,
}