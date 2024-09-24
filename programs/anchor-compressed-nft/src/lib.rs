

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
use mpl_bubblegum::instructions::VerifyLeafCpiBuilder;
use mpl_bubblegum::instructions::MintV1CpiBuilder;
use mpl_bubblegum::types::LeafSchema;
use std::str::FromStr;
use mpl_bubblegum::hash::hash_metadata;
use mpl_bubblegum::hash::hash_creators;

declare_id!("A6VX3rMDSXUvK4uDD8jf9thgt4qB1bv4U26V3KUHWC4W");

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
        let hash = solana_program::keccak::hashv(&[metadata.try_to_vec()?.as_slice()]);
        // Calculate new data hash.
        let metada_hash = solana_program::keccak::hashv(&[
            &hash.to_bytes(),
            &metadata.seller_fee_basis_points.to_le_bytes(),
        ])
        .to_bytes();
        msg!("before cpi and after metadata:{:?}",metadata);
        msg!("before cpi and metadata hash:{:?}",metada_hash);


    //     MintToCollectionV1CpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
    // .compression_program(&ctx.accounts.compression_program.to_account_info())
    // .leaf_delegate(&ctx.accounts.payer.to_account_info())
    // .leaf_owner(&ctx.accounts.payer.to_account_info())
    // .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
    // .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
    // .payer(&ctx.accounts.payer.to_account_info())
    // .system_program(&ctx.accounts.system_program.to_account_info())
    // .tree_config(&ctx.accounts.tree_authority.to_account_info())
    // .tree_creator_or_delegate(&ctx.accounts.pda.to_account_info())
    // .collection_authority(&ctx.accounts.pda.to_account_info())
    // .collection_authority_record_pda(Some(&ctx.accounts.bubblegum_program.to_account_info()))
    // .collection_mint(&ctx.accounts.collection_mint.to_account_info())
    // .collection_metadata(&ctx.accounts.collection_metadata.to_account_info())
    // .collection_edition(&ctx.accounts.edition_account.to_account_info()) 
    // .bubblegum_signer(&ctx.accounts.bubblegum_signer.to_account_info())  
    // .token_metadata_program(&ctx.accounts.token_metadata_program.to_account_info())
    // .system_program(&ctx.accounts.system_program.to_account_info())
    // .metadata(metadata)
    // .invoke_signed(signer_seeds);
    MintV1CpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
    .tree_config(&ctx.accounts.tree_authority.to_account_info())
    .leaf_owner(&ctx.accounts.payer.to_account_info())
    .leaf_delegate(&ctx.accounts.payer.to_account_info())
    .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
    .payer(&ctx.accounts.payer.to_account_info())
    .tree_creator_or_delegate(&ctx.accounts.pda.to_account_info())
    .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
    .compression_program(&ctx.accounts.compression_program.to_account_info())
    .system_program(&ctx.accounts.system_program.to_account_info())
    .metadata(metadata).invoke_signed(signer_seeds).unwrap();

    msg!("after cpi");

    

        Ok(())
    }

    /// Computes the hash of the metadata.
///
/// The hash is computed as the keccak256 hash of the metadata bytes, which is
/// then hashed with the `seller_fee_basis_points`.


    pub fn mint_if_creator_nft<'a, 'b, 'c, 'info>( ctx: Context<'a, 'b, 'c, 'info, MintIfCreatorNFT<'info>>,
    root: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    data_hash: [u8; 32]
) -> Result<()>{
    let signer_seeds: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[ctx.bumps.pda]]];
    msg!("before cpi mint if creator nft");
            let compression_program = &ctx.accounts.compression_program.to_account_info();
            let metadata_if = &ctx.accounts.collection_if_metadata;
            let asset_id_key = &ctx.accounts.asset_id.key();
            let creator_key = &ctx.accounts.creator.key();


            let verify_metadata = MetadataArgs {
                name: metadata_if.name.to_string(),
                symbol: metadata_if.symbol.to_string(),
                uri: metadata_if.uri.to_string(),
                collection: Some(Collection {
                    key: ctx.accounts.collection_if_mint.key(),
                    verified: false,
                }),
                primary_sale_happened: true,
                is_mutable: true,
                edition_nonce: None,
                token_standard: Some(TokenStandard::NonFungible),
                uses: None,
                token_program_version: TokenProgramVersion::Original,
                creators: vec![Creator {
                    address: *creator_key, // set creator as pda
                    verified: true,
                    share: 100,
                }],
                seller_fee_basis_points: 0,
            };

        

             let mut verify_metadata_hash: [u8; 32] = hash_metadata(&verify_metadata)?;
            // let mut metada_hash: [u8; 32] = hash_metadata(&metadata)?;
            let metadata_args_hash = solana_program::keccak::hashv(&[verify_metadata.try_to_vec()?.as_slice()]);
            let metadata_hash = solana_program::keccak::hashv(&[
          &metadata_args_hash.to_bytes(),
          &verify_metadata.seller_fee_basis_points.to_le_bytes(),
            ]);
            let data_vec: Vec<u8> = metadata_hash.0.try_to_vec()?;
            let mut verify_data_hash: [u8; 32] = [0; 32];
            verify_data_hash.copy_from_slice(&data_vec); 
            msg!("data_hash metad:{:?}",data_hash);
            msg!("verify metadata:{:?}",verify_data_hash);
            msg!("datahash metadata:{:?}",verify_metadata_hash);



            let leaf = LeafSchema::V1 {
                id: *asset_id_key,         // You need to name the field `id`
                owner: ctx.accounts.payer.key(),        // You need to name the field `owner`
                delegate:  ctx.accounts.payer.key(),  // You need to name the field `delegate`
                nonce: u64::from(index),  // Name the field `nonce`
                data_hash:verify_data_hash,                // If the variable name matches the field, you can omit the explicit name (this is shorthand)
                creator_hash,             // Same here, shorthand for `creator_hash: creator_hash`
            };


            msg!("before cpi verify leaf");

         let good =   VerifyLeafCpiBuilder::new(compression_program)
            .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
            .root(root)
            .leaf(leaf.hash())
            .index(index)
            .add_remaining_accounts(
                &ctx.remaining_accounts
                    .iter()
                    .map(|acc| (acc, false, false))
                    .collect::<Vec<_>>()
                ).invoke();

            // msg!("verify metadata:{:?}",good);

            MintV1CpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
            .tree_config(&ctx.accounts.tree_authority.to_account_info())
            .leaf_owner(&ctx.accounts.payer.to_account_info())
            .leaf_delegate(&ctx.accounts.payer.to_account_info())
            .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
            .payer(&ctx.accounts.payer.to_account_info())
            .tree_creator_or_delegate(&ctx.accounts.pda)
            .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
            .compression_program(&ctx.accounts.compression_program.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .metadata(verify_metadata).invoke_signed(signer_seeds).unwrap();
        
        Ok(())
    }



    pub fn burn_compressed_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, BurnCompressedNft<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        // remaining_accounts are the accounts that make up the required proof
        let remaining_accounts_len = ctx.remaining_accounts.len();
        let mut accounts = Vec::with_capacity(
            7 // space for the 7 AccountMetas that are always included in (below)
            + remaining_accounts_len,
        );
        accounts.extend(vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_delegate.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ]);

        let burn_discriminator: [u8; 8] = [116, 110, 29, 56, 107, 219, 42, 93];

        let mut data = Vec::with_capacity(
            8 // The length of burn_discriminator,
            + root.len()
            + data_hash.len()
            + creator_hash.len()
            + 8 // The length of the nonce
            + 8, // The length of the index
        );
        data.extend(burn_discriminator);
        data.extend(root);
        data.extend(data_hash);
        data.extend(creator_hash);
        data.extend(nonce.to_le_bytes());
        data.extend(index.to_le_bytes());

        let mut account_infos = Vec::with_capacity(
            7 // space for the 7 AccountInfos that are always included (below)
            + remaining_accounts_len,
        );
        account_infos.extend(vec![
            ctx.accounts.tree_authority.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_delegate.to_account_info(),
            ctx.accounts.merkle_tree.to_account_info(),
            ctx.accounts.log_wrapper.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]);

        // Add "accounts" (hashes) that make up the merkle proof from the remaining accounts.
        for acc in ctx.remaining_accounts.iter() {
            accounts.push(AccountMeta::new_readonly(acc.key(), false));
            account_infos.push(acc.to_account_info());
        }

        let instruction = solana_program::instruction::Instruction {
            program_id: ctx.accounts.bubblegum_program.key(),
            accounts,
            data,
        };

        msg!("manual cpi call to bubblegum program burn instruction");
        solana_program::program::invoke(&instruction, &account_infos[..])?;

        // // // Below not working
        // // // Error Code: LeafAuthorityMustSign. Error Number: 6025. Error Message: This transaction must be signed by either the leaf owner or leaf delegate.'
        // let cpi_ctx = CpiContext::new(
        //     ctx.accounts.bubblegum_program.to_account_info(),
        //     Burn {
        //         tree_authority: ctx.accounts.tree_authority.to_account_info(),
        //         leaf_owner: ctx.accounts.payer.to_account_info(),
        //         leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
        //         merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        //         log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
        //         compression_program: ctx.accounts.compression_program.to_account_info(),
        //         system_program: ctx.accounts.system_program.to_account_info(),
        //     },
        // )
        // .with_remaining_accounts(ctx.remaining_accounts.to_vec());

        // burn(cpi_ctx, root, data_hash, creator_hash, nonce, index)?;

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

#[derive(Accounts)]
pub struct BurnCompressedNft<'info> {
    #[account(mut)]
    pub leaf_owner: Signer<'info>,

    #[account(mut)]
    pub leaf_delegate: Signer<'info>,

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
    pub compression_program: UncheckedAccount<'info>,
    pub bubblegum_program: UncheckedAccount<'info>,
    pub system_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MintIfCreatorNFT<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,
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
    pub collection_if_metadata: Account<'info, MetadataAccount>,
    pub collection_if_mint: Account<'info, Mint>,
    pub creator:UncheckedAccount<'info>,
    pub asset_id:UncheckedAccount<'info>

}