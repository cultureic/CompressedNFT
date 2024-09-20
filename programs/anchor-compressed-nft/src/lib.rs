

use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use anchor_spl::{
    metadata::{Metadata, MetadataAccount},
    token::Mint,
};
use mpl_bubblegum::InstructionName::CreateTree;
use mpl_bubblegum::types::MetadataArgs;
use mpl_bubblegum::types::Collection;
use mpl_bubblegum::types::TokenStandard;
use mpl_bubblegum::types::TokenProgramVersion;
use mpl_bubblegum::types::Creator;
use mpl_bubblegum::instructions::MintToCollectionV1;
use mpl_bubblegum::instructions::CreateTreeConfigCpiBuilder;
use mpl_bubblegum::programs::MPL_BUBBLEGUM_ID;
use mpl_bubblegum::instructions::MintToCollectionV1CpiBuilder;


declare_id!("HTFevMxdBb5rjBJrE3ATtWtPZc9yvfmnxS34kU843pFf");

pub const SEED: &str = "AUTH";

#[program]
pub mod anchor_compressed_nft {

    use super::*;

    pub fn anchor_create_tree(
        ctx: Context<AnchorCreateTree>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[ctx.bumps.pda]]];
        msg!("before cpi");

    let system_program = ctx.accounts.system_program.to_account_info();
    let payer = ctx.accounts.payer.to_account_info();
    let merkle = ctx.accounts.merkle_tree.to_account_info();
    let logw = ctx.accounts.log_wrapper.to_account_info();
    let tree_authority = ctx.accounts.tree_authority.to_account_info();
    let bubblegum = ctx.accounts.bubblegum_program.to_account_info();
    let compressionbinding = &ctx.accounts.compression_program.to_account_info();
    // msg!("PDA: {:?}", signer_seeds);
    // msg!("payer: {:?}", payer);
    // msg!("merkle: {:?}", merkle);
    // msg!("logw: {:?}", logw);
    // msg!("tree_authority: {:?}", tree_authority);
    // msg!("bubblegum: {:?}", bubblegum);
    // msg!("compressionbinding: {:?}", compressionbinding);


    //msg!("bubblegum:{}",bubblegum);
    msg!("before cpi and after accounts");
     CreateTreeConfigCpiBuilder::new(&bubblegum)
    .compression_program(&compressionbinding)
    .tree_config(&tree_authority)
    .log_wrapper(&logw)
    .merkle_tree(&merkle)
    .payer(&payer)
    .system_program(&system_program)
    .max_depth(max_depth)
    .max_buffer_size(max_buffer_size)
    .tree_creator(&ctx.accounts.pda.to_account_info())
    .invoke_signed(signer_seeds);
        msg!("after cpi");
// performs the CPI
        Ok(())
    }

    pub fn mint_compressed_nft(ctx: Context<MintCompressedNft>) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[ctx.bumps.pda]]];

        // use collection nft metadata as the metadata for the compressed nft
        let metadata_account = &ctx.accounts.collection_metadata;

        let metadata = MetadataArgs {
            name: metadata_account.name.to_string(),
            symbol: metadata_account.symbol.to_string(),
            uri: metadata_account.uri.to_string(),
            collection: Some(Collection {
                key: ctx.accounts.collection_mint.key(),
                verified: false,
            }),
            primary_sale_happened: true,
            is_mutable: true,
            edition_nonce: None,
            token_standard: Some(TokenStandard::NonFungible),
            uses: None,
            token_program_version: TokenProgramVersion::Original,
            creators: vec![Creator {
                address: ctx.accounts.pda.key(), // set creator as pda
                verified: true,
                share: 100,
            }],
            seller_fee_basis_points: 0,
        };
        msg!("before cpi and after metadata");

        MintToCollectionV1CpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
    .compression_program(&ctx.accounts.compression_program.to_account_info())
    .leaf_delegate(&ctx.accounts.payer.to_account_info())
    .leaf_owner(&ctx.accounts.payer.to_account_info())
    .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
    .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
    .payer(&ctx.accounts.payer.to_account_info())
    .system_program(&ctx.accounts.system_program.to_account_info())
    .tree_config(&ctx.accounts.tree_authority.to_account_info())
    .tree_creator_or_delegate(&ctx.accounts.pda.to_account_info())
    .collection_authority(&ctx.accounts.pda.to_account_info())
    .collection_authority_record_pda(Some(&ctx.accounts.bubblegum_program.to_account_info()))
    .collection_mint(&ctx.accounts.collection_mint.to_account_info())
    .collection_metadata(&ctx.accounts.collection_metadata.to_account_info())
    .collection_edition(&ctx.accounts.edition_account.to_account_info()) 
    .bubblegum_signer(&ctx.accounts.bubblegum_signer.to_account_info())  
    .token_metadata_program(&ctx.accounts.token_metadata_program.to_account_info())
    .system_program(&ctx.accounts.system_program.to_account_info())
    .metadata(metadata)
    .invoke_signed(signer_seeds);

    msg!("after cpi");

    

        Ok(())
    }

  
 }

#[derive(Accounts)]
pub struct AnchorCreateTree<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK:
    #[account(
        seeds = [SEED.as_bytes()],
        bump,
    )]
    pub pda: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    pub tree_authority: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    pub log_wrapper: UncheckedAccount<'info>,
    pub system_program: UncheckedAccount<'info>,
    pub bubblegum_program: UncheckedAccount<'info>,
    pub compression_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MintCompressedNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK:
    #[account(
        seeds = [SEED.as_bytes()],
        bump,
    )]
    pub pda: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        seeds = ["collection_cpi".as_bytes()],
        seeds::program = bubblegum_program.key(),
        bump,
    )]
    pub bubblegum_signer: UncheckedAccount<'info>,

    pub log_wrapper: UncheckedAccount<'info>,
    pub compression_program: UncheckedAccount<'info>,
    pub bubblegum_program: UncheckedAccount<'info>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,

    pub collection_mint: Account<'info, Mint>,
    #[account(mut)]
    pub collection_metadata: Account<'info, MetadataAccount>,
    /// CHECK:
    pub edition_account: UncheckedAccount<'info>,
}

