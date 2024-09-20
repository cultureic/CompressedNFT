
// #![allow(clippy::result_large_err)]

// use {
//     anchor_lang::prelude::*,
//     anchor_spl::{
//         associated_token::AssociatedToken,
//         metadata::{
//             create_master_edition_v3, create_metadata_accounts_v3,
//             mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3,
//             Metadata,
//         },
//         token::{mint_to, Mint, MintTo, Token, TokenAccount},
//     },
// };

// declare_id!("DFbmYptf2Fp4X9dCL4yKifYFn8zR859Cnvf3fu1XDhWg");

// #[program]
// pub mod nft_minter {
//     use super::*;

//     pub fn mint_nft(
//         ctx: Context<CreateToken>,
//         nft_name: String,
//         nft_symbol: String,
//         nft_uri: String,
//     ) -> Result<()> {
//         msg!("Minting Token");
//         // Cross Program Invocation (CPI)
//         // Invoking the mint_to instruction on the token program
//         mint_to(
//             CpiContext::new(
//                 ctx.accounts.token_program.to_account_info(),
//                 MintTo {
//                     mint: ctx.accounts.mint_account.to_account_info(),
//                     to: ctx.accounts.associated_token_account.to_account_info(),
//                     authority: ctx.accounts.payer.to_account_info(),
//                 },
//             ),
//             1,
//         )?;

//         msg!("Creating metadata account");
//         // Cross Program Invocation (CPI)
//         // Invoking the create_metadata_account_v3 instruction on the token metadata program
//         create_metadata_accounts_v3(
//             CpiContext::new(
//                 ctx.accounts.token_metadata_program.to_account_info(),
//                 CreateMetadataAccountsV3 {
//                     metadata: ctx.accounts.metadata_account.to_account_info(),
//                     mint: ctx.accounts.mint_account.to_account_info(),
//                     mint_authority: ctx.accounts.payer.to_account_info(),
//                     update_authority: ctx.accounts.payer.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//             ),
//             DataV2 {
//                 name: nft_name,
//                 symbol: nft_symbol,
//                 uri: nft_uri,
//                 seller_fee_basis_points: 0,
//                 creators: None,
//                 collection: None,
//                 uses: None,
//             },
//             false, // Is mutable
//             true,  // Update authority is signer
//             None,  // Collection details
//         )?;

//         msg!("Creating master edition account");
//         // Cross Program Invocation (CPI)
//         // Invoking the create_master_edition_v3 instruction on the token metadata program
//         create_master_edition_v3(
//             CpiContext::new(
//                 ctx.accounts.token_metadata_program.to_account_info(),
//                 CreateMasterEditionV3 {
//                     edition: ctx.accounts.edition_account.to_account_info(),
//                     mint: ctx.accounts.mint_account.to_account_info(),
//                     update_authority: ctx.accounts.payer.to_account_info(),
//                     mint_authority: ctx.accounts.payer.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     metadata: ctx.accounts.metadata_account.to_account_info(),
//                     token_program: ctx.accounts.token_program.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//             ),
//             None, // Max Supply
//         )?;

//         msg!("NFT minted successfully.");

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct CreateToken<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,

//     /// CHECK: Validate address by deriving pda
//     #[account(
//         mut,
//         seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
//         bump,
//         seeds::program = token_metadata_program.key(),
//     )]
//     pub metadata_account: UncheckedAccount<'info>,

//     /// CHECK: Validate address by deriving pda
//     #[account(
//         mut,
//         seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
//         bump,
//         seeds::program = token_metadata_program.key(),
//     )]
//     pub edition_account: UncheckedAccount<'info>,

//     // Create new mint account, NFTs have 0 decimals
//     #[account(
//         init,
//         payer = payer,
//         mint::decimals = 0,
//         mint::authority = payer.key(),
//         mint::freeze_authority = payer.key(),
//     )]
//     pub mint_account: Account<'info, Mint>,

//     // Create associated token account, if needed
//     // This is the account that will hold the NFT
//     #[account(
//         init_if_needed,
//         payer = payer,
//         associated_token::mint = mint_account,
//         associated_token::authority = payer,
//     )]
//     pub associated_token_account: Account<'info, TokenAccount>,

//     pub token_program: Program<'info, Token>,
//     pub token_metadata_program: Program<'info, Metadata>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
// }

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



        // create_tree(
        //     CpiContext::new_with_signer(
        //         ctx.accounts.bubblegum_program.to_account_info(),
        //         CreateTree {
        //             tree_authority: ctx.accounts.tree_authority.to_account_info(),
        //             merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        //             payer: ctx.accounts.payer.to_account_info(),
        //             tree_creator: ctx.accounts.pda.to_account_info(), // set creator as pda
        //             log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
        //             compression_program: ctx.accounts.compression_program.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //         },
        //         signer_seeds,
        //     ),
        //     max_depth,
        //     max_buffer_size,
        //     Option::from(false),
        // )?;
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

        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.bubblegum_program.to_account_info(),
        //     MintToCollectionV1 {
        //         tree_authority: ctx.accounts.tree_authority.to_account_info(),
        //         leaf_owner: ctx.accounts.payer.to_account_info(),
        //         leaf_delegate: ctx.accounts.payer.to_account_info(),
        //         merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        //         payer: ctx.accounts.payer.to_account_info(),
        //         tree_delegate: ctx.accounts.pda.to_account_info(), // tree delegate is pda, required as a signer
        //         collection_authority: ctx.accounts.pda.to_account_info(), // collection authority is pda (nft metadata update authority)
        //         collection_authority_record_pda: ctx.accounts.bubblegum_program.to_account_info(),
        //         collection_mint: ctx.accounts.collection_mint.to_account_info(), // collection nft mint account
        //         collection_metadata: ctx.accounts.collection_metadata.to_account_info(), // collection nft metadata account
        //         edition_account: ctx.accounts.edition_account.to_account_info(), // collection nft master edition account
        //         bubblegum_signer: ctx.accounts.bubblegum_signer.to_account_info(),
        //         log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
        //         compression_program: ctx.accounts.compression_program.to_account_info(),
        //         token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
        //         system_program: ctx.accounts.system_program.to_account_info(),
        //     },
        //     signer_seeds,
        // );

        // mint_to_collection_v1(cpi_ctx, metadata)?;

        Ok(())
    }

    // pub fn burn_compressed_nft<'a, 'b, 'c, 'info>(
    //     ctx: Context<'a, 'b, 'c, 'info, BurnCompressedNft<'info>>,
    //     root: [u8; 32],
    //     data_hash: [u8; 32],
    //     creator_hash: [u8; 32],
    //     nonce: u64,
    //     index: u32,
    // ) -> Result<()> {
    //     // remaining_accounts are the accounts that make up the required proof
    //     let remaining_accounts_len = ctx.remaining_accounts.len();
    //     let mut accounts = Vec::with_capacity(
    //         7 // space for the 7 AccountMetas that are always included in (below)
    //         + remaining_accounts_len,
    //     );
    //     accounts.extend(vec![
    //         AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
    //         AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
    //         AccountMeta::new_readonly(ctx.accounts.leaf_delegate.key(), false),
    //         AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
    //         AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
    //         AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
    //         AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
    //     ]);

    //     let burn_discriminator: [u8; 8] = [116, 110, 29, 56, 107, 219, 42, 93];

    //     let mut data = Vec::with_capacity(
    //         8 // The length of burn_discriminator,
    //         + root.len()
    //         + data_hash.len()
    //         + creator_hash.len()
    //         + 8 // The length of the nonce
    //         + 8, // The length of the index
    //     );
    //     data.extend(burn_discriminator);
    //     data.extend(root);
    //     data.extend(data_hash);
    //     data.extend(creator_hash);
    //     data.extend(nonce.to_le_bytes());
    //     data.extend(index.to_le_bytes());

    //     let mut account_infos = Vec::with_capacity(
    //         7 // space for the 7 AccountInfos that are always included (below)
    //         + remaining_accounts_len,
    //     );
    //     account_infos.extend(vec![
    //         ctx.accounts.tree_authority.to_account_info(),
    //         ctx.accounts.leaf_owner.to_account_info(),
    //         ctx.accounts.leaf_delegate.to_account_info(),
    //         ctx.accounts.merkle_tree.to_account_info(),
    //         ctx.accounts.log_wrapper.to_account_info(),
    //         ctx.accounts.compression_program.to_account_info(),
    //         ctx.accounts.system_program.to_account_info(),
    //     ]);

    //     // Add "accounts" (hashes) that make up the merkle proof from the remaining accounts.
    //     for acc in ctx.remaining_accounts.iter() {
    //         accounts.push(AccountMeta::new_readonly(acc.key(), false));
    //         account_infos.push(acc.to_account_info());
    //     }

    //     let instruction = solana_program::instruction::Instruction {
    //         program_id: ctx.accounts.bubblegum_program.key(),
    //         accounts,
    //         data,
    //     };

    //     msg!("manual cpi call to bubblegum program burn instruction");
    //     solana_program::program::invoke(&instruction, &account_infos[..])?;

    //     // // // Below not working
    //     // // // Error Code: LeafAuthorityMustSign. Error Number: 6025. Error Message: This transaction must be signed by either the leaf owner or leaf delegate.'
    //     // let cpi_ctx = CpiContext::new(
    //     //     ctx.accounts.bubblegum_program.to_account_info(),
    //     //     Burn {
    //     //         tree_authority: ctx.accounts.tree_authority.to_account_info(),
    //     //         leaf_owner: ctx.accounts.leaf_owner.to_account_info(),
    //     //         leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
    //     //         merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
    //     //         log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
    //     //         compression_program: ctx.accounts.compression_program.to_account_info(),
    //     //         system_program: ctx.accounts.system_program.to_account_info(),
    //     //     },
    //     // )
    //     // .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    //     // burn(cpi_ctx, root, data_hash, creator_hash, nonce, index)?;

    //     Ok(())
    // }

//     pub fn transfer_compressed_nft<'a, 'b, 'c, 'info>(
//         ctx: Context<'a, 'b, 'c, 'info, TransferCompressedNft<'info>>,
//         root: [u8; 32],
//         data_hash: [u8; 32],
//         creator_hash: [u8; 32],
//         nonce: u64,
//         index: u32,
//     ) -> Result<()> {
//         // remaining_accounts are the accounts that make up the required proof
//         let remaining_accounts_len = ctx.remaining_accounts.len();
//         let mut accounts = Vec::with_capacity(
//             8 // space for the 8 AccountMetas that are always included in (below)
//             + remaining_accounts_len,
//         );
//         accounts.extend(vec![
//             AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
//             AccountMeta::new_readonly(ctx.accounts.leaf_delegate.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.new_leaf_owner.key(), false),
//             AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
//         ]);

//         let transfer_discriminator: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];

//         let mut data = Vec::with_capacity(
//             8 // The length of transfer_discriminator,
//             + root.len()
//             + data_hash.len()
//             + creator_hash.len()
//             + 8 // The length of the nonce
//             + 8, // The length of the index
//         );
//         data.extend(transfer_discriminator);
//         data.extend(root);
//         data.extend(data_hash);
//         data.extend(creator_hash);
//         data.extend(nonce.to_le_bytes());
//         data.extend(index.to_le_bytes());

//         let mut account_infos = Vec::with_capacity(
//             8 // space for the 8 AccountInfos that are always included (below)
//             + remaining_accounts_len,
//         );
//         account_infos.extend(vec![
//             ctx.accounts.tree_authority.to_account_info(),
//             ctx.accounts.leaf_owner.to_account_info(),
//             ctx.accounts.leaf_delegate.to_account_info(),
//             ctx.accounts.new_leaf_owner.to_account_info(),
//             ctx.accounts.merkle_tree.to_account_info(),
//             ctx.accounts.log_wrapper.to_account_info(),
//             ctx.accounts.compression_program.to_account_info(),
//             ctx.accounts.system_program.to_account_info(),
//         ]);

//         // Add "accounts" (hashes) that make up the merkle proof from the remaining accounts.
//         for acc in ctx.remaining_accounts.iter() {
//             accounts.push(AccountMeta::new_readonly(acc.key(), false));
//             account_infos.push(acc.to_account_info());
//         }

//         let instruction = solana_program::instruction::Instruction {
//             program_id: ctx.accounts.bubblegum_program.key(),
//             accounts,
//             data,
//         };

//         msg!("manual cpi call to bubblegum program transfer instruction");
//         solana_program::program::invoke(&instruction, &account_infos[..])?;

//         // // Below not working
//         // // Error Code: LeafAuthorityMustSign. Error Number: 6025. Error Message: This transaction must be signed by either the leaf owner or leaf delegate.'
//         // msg!("remaining_accounts: {:?}", ctx.remaining_accounts.to_vec());

//         // let mut remaining_account_infos: Vec<AccountInfo> = Vec::new();
//         // for acc in ctx.remaining_accounts.iter() {
//         //     remaining_account_infos.push(acc.to_account_info());
//         // }

//         // let cpi_ctx = CpiContext::new(
//         //     ctx.accounts.bubblegum_program.to_account_info(),
//         //     Transfer {
//         //         tree_authority: ctx.accounts.tree_authority.to_account_info(),
//         //         leaf_owner: ctx.accounts.leaf_owner.to_account_info(),
//         //         leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
//         //         new_leaf_owner: ctx.accounts.new_leaf_owner.to_account_info(),
//         //         merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
//         //         log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
//         //         compression_program: ctx.accounts.compression_program.to_account_info(),
//         //         system_program: ctx.accounts.system_program.to_account_info(),
//         //     },
//         // )
//         // // .with_remaining_accounts(ctx.remaining_accounts.to_vec());
//         // .with_remaining_accounts(remaining_account_infos);

//         // transfer(cpi_ctx, root, data_hash, creator_hash, nonce, index)?;
//         Ok(())
//     }
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

// #[derive(Accounts)]
// pub struct BurnCompressedNft<'info,Bubblegum> {
//     #[account(mut)]
//     pub leaf_owner: Signer<'info>,

//     #[account(mut)]
//     pub leaf_delegate: Signer<'info>,

//     /// CHECK:
//     #[account(
//         mut,
//         seeds = [merkle_tree.key().as_ref()],
//         bump,
//         seeds::program = bubblegum_program.key()
//     )]
//     pub tree_authority: UncheckedAccount<'info>,

//     /// CHECK:
//     #[account(mut)]
//     pub merkle_tree: UncheckedAccount<'info>,

//     pub log_wrapper: Program<'info, Noop>,
//     pub compression_program: Program<'info, SplAccountCompression>,
//     pub bubblegum_program: Program<'info, Bubblegum>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct TransferCompressedNft<'info,Bubblegum> {
//     #[account(mut)]
//     pub leaf_owner: Signer<'info>,

//     #[account(mut)]
//     pub leaf_delegate: Signer<'info>,

//     /// CHECK:
//     #[account(
//         mut,
//         seeds = [merkle_tree.key().as_ref()],
//         bump,
//         seeds::program = bubblegum_program.key()
//     )]
//     pub tree_authority: UncheckedAccount<'info>,

//     /// CHECK:
//     #[account(mut)]
//     pub merkle_tree: UncheckedAccount<'info>,

//     /// CHECK:
//     #[account(mut)]
//     pub new_leaf_owner: UncheckedAccount<'info>,

//     pub log_wrapper: Program<'info, Noop>,
//     pub compression_program: Program<'info, SplAccountCompression>,
//     pub bubblegum_program: Program<'info, Bubblegum>,
//     pub system_program: Program<'info, System>,
// }


// solana-program = "=1.18.3"
// winnow="=0.4.1"
// toml_datetime="=0.6.5"
// mpl-bubblegum = "1.4.0"
// mpl-token-metadata = "4.1.2"
// spl-account-compression = "0.4.0"
// borsh = "1.5.1"
// spl-token-2022 = "0.8.0"