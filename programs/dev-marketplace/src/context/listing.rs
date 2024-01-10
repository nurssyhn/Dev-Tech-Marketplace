pub use anchor_lang::prelude::*;
pub use solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID;

use anchor_spl::{
    token::{Mint, TokenAccount},
    metadata::{Metadata, MetadataAccount, MasterEditionAccount,
    mpl_token_metadata::{
        instructions::{TransferCpi, TransferCpiAccounts, TransferInstructionArgs},
    types::{TokenStandard,Collection},
}},
    associated_token::{AssociatedToken, AssociatedTokenAccount},
};

pub use anchor_spl::token::Token;

pub use crate::state::*;
pub use crate::errors::*;

#[derive(Accounts)]
pub struct List <'info>{
   #[account(mut)]
   pub lister: Signer<'info>,
   #[account(
    mut,
    associated_token::mint = nft.key(),
    associated_token::authority = lister,
)]
pub lister_ata: Account<'info, TokenAccount>,

#[account(
seeds = [b"DevMarketplace", DevMarketplace.name.as_str().as_bytes()],
bump,
)]
pub DevMarketplace: Account<'info, DevMarketplace>,
#[account(
    init, 
    payer = lister, 
    seeds = [b"listing", DevMarketplace.key().as_ref()],
    bump,
    space = LISTING::INIT_SPACE,
)]
pub listing: Account<'info, Listing>,
#[account(
    init, 
    payer = lister,
    associated_token::mint = nft,
    associated_token::authority = listing,
    
)]
pub listing_vault: Account<'info, TokenAccount>,

pub collection: Account<'info, Mint>,
#[account(mut)]
pub nft: Account<'info, Mint>,
#[account(mut)]
pub metadata: Account<'info, Metadata>,
pub edition: Account<'info, MasterEditionAccount>,

#[account(address = INSTRUCTIONS_ID)]
///CHECK: No need to check it out
pub sysvar_instruction: Account<'info, SystemInstructions>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub token_metadata_program: Account<'info, Metadata>,
pub token_program: Program<'info, Token>,
pub system_program: Program<'info, System>,
}

impl <'info> List <'info> {
    pub fn list(
    &mut self,
    price: u64,
    ) -> Result <()> {

            #[derive(Accounts)]
            pub struct List<'info> {
                #[account(mut)]
                pub lister: Signer<'info>,
                #[account(
                    mut,
                    associated_token::mint = nft.key(),
                    associated_token::authority = lister,
                )]
                pub lister_ata: Account<'info, TokenAccount>,

                #[account(
                    seeds = [b"DevMarketplace", DevMarketplace.name.as_str().as_bytes()],
                    bump,
                )]
                pub DevMarketplace: Account<'info, DevMarketplace>,
                #[account(
                    init,
                    payer = lister,
                    seeds = [b"listing", DevMarketplace.key().as_ref()],
                    bump,
                    space = LISTING::INIT_SPACE,
                )]
                pub listing: Account<'info, Listing>,
                #[account(
                    init,
                    payer = lister,
                    associated_token::mint = nft,
                    associated_token::authority = listing,
                )]
                pub listing_vault: Account<'info, TokenAccount>,

                pub collection: Account<'info, Mint>,
                #[account(mut)]
                pub nft: Account<'info, Mint>,
                #[account(mut)]
                pub metadata: Account<'info, Metadata>,
                pub edition: Account<'info, MasterEditionAccount>,

                #[account(address = INSTRUCTIONS_ID)]
                ///CHECK: No need to check it out
                pub sysvar_instruction: Account<'info, SystemInstructions>,
                pub associated_token_program: Program<'info, AssociatedToken>,
                pub token_metadata_program: Account<'info, Metadata>,
                pub token_program: Program<'info, Token>,
                pub system_program: Program<'info, System>,
            }

            impl<'info> List<'info> {
                pub fn list(&mut self, price: u64) -> Result<()> {
                    require!(
                        self.metadata.token_standard.clone().unwrap() == TokenStandard::NonFungible,
                        DevMarketplaceError::InvalidTokenStandard
                    );
                    require!(
                        self.metadata.data.collection.clone().unwrap()
                            == Collection {
                                verified: true,
                                pubkey: self.collection.key(),
                                owner: None, 
                                services: None, 
                                availability: None, 
                            },
                        DevMarketplaceError::InvalidCollection
                    );

                    self.listing.set_inner(Listing {
                        lister: self.lister.to_account_info(),
                        nft: self.nft.to_account_info(),
                        collection: self.collection.to_account_info(),
                        price,
                        owner: None, 
                        services: None, 
                        availability: None, 
                    });

                    let transfer_program: AccountInfo = self.token_program.to_account_info();
                    let token = &self.lister_ata.to_account_info();
                    let token_owner = &self.lister.to_account_info();
                    let destination_token = &self.listing_vault.to_account_info();
                    let destination_owner = &self.listing.to_account_info();
                    let mint = &self.nft.to_account_info();

                }
            }
    let metadata = &self.metadata.to_account_info();
    let edition = Some(&self.edition.to_account_info());
    let token_record = None;        
    let destination_token_record = None;
    let authority = &self.lister.to_account_info();
    let payer = &self.lister.to_account_info();
    let system_program = &self.system_program.to_account_info();
    let sysvar_instructions = &self.sysvar_instruction.to_account_info();
    let spl_token_program = &self.token_program.to_account_info();
    let spl_ata_program = &self.associated_token_program.to_account_info();
    let authorization_rules_program = None;
    let authorization_rules = None;

    let transfer_cpi = TransferCpi::new(
        &transfer_program,
        TransferCpiAccounts {
            token: token_owner,
            token_owner,
            destination_token,
            destination_owner,
            mint,
            metadata,
            edition,
            token_record,
            destination_token_record, 
            authority,
            payer,
            system_program,
            sysvar_instructions,
            spl_token_program,
            spl_ata_program,
            authorization_rules_program,
            authorization_rules,
        },
        TransferInstructionArgs {
            transfer_args: TransferArgs::V1{
                amount: 1,
                authorization_rules: None,
            },
        }
    );

    transfer_cpi.invoke()?;
    
    Ok(())
    }   
}