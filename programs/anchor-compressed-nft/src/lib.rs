use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{Metadata, MetadataAccount},
    token::Mint,
};
use mpl_bubblegum::{
    cpi::{
        accounts::{Burn, CreateTree, MintToCollectionV1, Transfer},
        burn, create_tree, mint_to_collection_v1, transfer,
    },
    program::Bubblegum,
    state::metaplex_adapter::{
        Collection, Creator, MetadataArgs, TokenProgramVersion, TokenStandard,
    },
};
use spl_account_compression::{program::SplAccountCompression, Noop};

declare_id!("AYorEHWdAA7SLzWgQfuv6kypdzriqCeG7GrGifa7c4Kp");

pub const SEED: &str = "AUTH";

#[program]
pub mod anchor_compressed_nft {

    use super::*;

    pub fn anchor_create_tree(
        ctx: Context<AnchorCreateTree>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[*ctx.bumps.get("pda").unwrap()]]];

        create_tree(
            CpiContext::new_with_signer(
                ctx.accounts.bubblegum_program.to_account_info(),
                CreateTree {
                    tree_authority: ctx.accounts.tree_authority.to_account_info(),
                    merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    tree_creator: ctx.accounts.pda.to_account_info(), // set creator as pda
                    log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                    compression_program: ctx.accounts.compression_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
                signer_seeds,
            ),
            max_depth,
            max_buffer_size,
            Option::from(false),
        )?;
        Ok(())
    }

    pub fn mint_compressed_nft(ctx: Context<MintCompressedNft>) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[SEED.as_bytes(), &[*ctx.bumps.get("pda").unwrap()]]];

        // use collection nft metadata as the metadata for the compressed nft
        let metadata_account = &ctx.accounts.collection_metadata;

        let metadata = MetadataArgs {
            name: metadata_account.data.name.to_string(),
            symbol: metadata_account.data.symbol.to_string(),
            uri: metadata_account.data.uri.to_string(),
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

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.bubblegum_program.to_account_info(),
            MintToCollectionV1 {
                tree_authority: ctx.accounts.tree_authority.to_account_info(),
                leaf_owner: ctx.accounts.payer.to_account_info(),
                leaf_delegate: ctx.accounts.payer.to_account_info(),
                merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                tree_delegate: ctx.accounts.pda.to_account_info(), // tree delegate is pda, required as a signer
                collection_authority: ctx.accounts.pda.to_account_info(), // collection authority is pda (nft metadata update authority)
                collection_authority_record_pda: ctx.accounts.bubblegum_program.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(), // collection nft mint account
                collection_metadata: ctx.accounts.collection_metadata.to_account_info(), // collection nft metadata account
                edition_account: ctx.accounts.edition_account.to_account_info(), // collection nft master edition account
                bubblegum_signer: ctx.accounts.bubblegum_signer.to_account_info(),
                log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                compression_program: ctx.accounts.compression_program.to_account_info(),
                token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            signer_seeds,
        );

        mint_to_collection_v1(cpi_ctx, metadata)?;

        Ok(())
    }

    // Not working idk why even though payer is signer
    // Error Code: LeafAuthorityMustSign. Error Number: 6025. Error Message: This transaction must be signed by either the leaf owner or leaf delegate.'
    pub fn burn_compressed_nft(
        ctx: Context<BurnCompressedNft>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.bubblegum_program.to_account_info(),
            Burn {
                tree_authority: ctx.accounts.tree_authority.to_account_info(),
                leaf_owner: ctx.accounts.leaf_owner.to_account_info(),
                leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
                merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                compression_program: ctx.accounts.compression_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );

        burn(cpi_ctx, root, data_hash, creator_hash, nonce, index)?;

        Ok(())
    }

    // Error Code: LeafAuthorityMustSign. Error Number: 6025. Error Message: This transaction must be signed by either the leaf owner or leaf delegate.'
    pub fn transfer_compressed_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, TransferCompressedNft<'info>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        msg!("remaining_accounts: {:?}", ctx.remaining_accounts.to_vec());

        let cpi_ctx = CpiContext::new(
            ctx.accounts.bubblegum_program.to_account_info(),
            Transfer {
                tree_authority: ctx.accounts.tree_authority.to_account_info(),
                leaf_owner: ctx.accounts.leaf_owner.to_account_info(),
                leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
                new_leaf_owner: ctx.accounts.new_leaf_owner.to_account_info(),
                merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
                log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
                compression_program: ctx.accounts.compression_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        )
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

        transfer(cpi_ctx, root, data_hash, creator_hash, nonce, index)?;

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
    pub log_wrapper: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
    pub bubblegum_program: Program<'info, Bubblegum>,
    pub compression_program: Program<'info, SplAccountCompression>,
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

    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, Bubblegum>,
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
    pub payer: Signer<'info>,

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

    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, Bubblegum>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferCompressedNft<'info> {
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

    /// CHECK:
    #[account(mut)]
    pub new_leaf_owner: UncheckedAccount<'info>,

    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, Bubblegum>,
    pub system_program: Program<'info, System>,
}
