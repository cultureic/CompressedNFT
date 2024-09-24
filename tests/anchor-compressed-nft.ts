import * as anchor from "@project-serum/anchor"
import { AnchorCompressedNft } from "../target/types/anchor_compressed_nft"
import { Program } from "@project-serum/anchor"
import {
  AccountMeta,
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js"
import {
  ConcurrentMerkleTreeAccount,
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
  ValidDepthSizePair,
  createAllocTreeIx,
} from "@solana/spl-account-compression"
import { PROGRAM_ID as BUBBLEGUM_PROGRAM_ID } from "@metaplex-foundation/mpl-bubblegum"
import {
  Metaplex,
  keypairIdentity,
  CreateNftOutput,
  InvalidJsonStringError,
} from "@metaplex-foundation/js"
import { assert } from "chai"
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"
import { extractAssetId, heliusApi } from "../utils/utils"

import { readFileSync } from 'fs';
import { publicKey } from "@project-serum/anchor/dist/cjs/utils"

const secretKey = Uint8Array.from(JSON.parse(readFileSync('././tester.json', 'utf8')));
const Testerkeypair = Keypair.fromSecretKey(secretKey);

describe("anchor-compressed-nft", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const wallet = provider.wallet as anchor.Wallet
  const program = anchor.workspace
    .AnchorCompressedNft as Program<AnchorCompressedNft>
  console.log("programid", program.programId)
  // const connection = program.provider.connection
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed")

  const metaplex = Metaplex.make(connection).use(keypairIdentity(wallet.payer))

  // keypair for tree
  const merkleTree = Keypair.generate()
  const list = Keypair.generate()

  // tree authority
  const [treeAuthority] = PublicKey.findProgramAddressSync(
    [merkleTree.publicKey.toBuffer()],
    BUBBLEGUM_PROGRAM_ID
  )

  // pda "tree creator", allows our program to update the tree
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("AUTH")],
    program.programId
  )

  const [bubblegumSigner] = PublicKey.findProgramAddressSync(
    [Buffer.from("collection_cpi", "utf8")],
    BUBBLEGUM_PROGRAM_ID
  )

  const maxDepthSizePair: ValidDepthSizePair = {
    maxDepth: 14,
    maxBufferSize: 64,
  }
  const canopyDepth = maxDepthSizePair.maxDepth - 5

  const metadata = {
    uri: "https://arweave.net/h19GMcMz7RLDY7kAHGWeWolHTmO83mLLMNPzEkF32BQ",
    name: "Collection",
    symbol: "COOL",
  }

  let collectionNft: CreateNftOutput
  let assetId: PublicKey
  let assetId2: PublicKey

  before(async () => {
    // Create collection nft
    collectionNft = await metaplex.nfts().create({
      uri: metadata.uri,
      name: metadata.name,
      sellerFeeBasisPoints: 0,
      isCollection: true,
    })

    // transfer collection nft metadata update authority to pda
    await metaplex.nfts().update({
      nftOrSft: collectionNft.nft,
      updateAuthority: wallet.payer,
      newUpdateAuthority: pda,
    })

    // instruction to create new account with required space for tree
    const allocTreeIx = await createAllocTreeIx(
      connection,
      merkleTree.publicKey,
      wallet.publicKey,
      maxDepthSizePair,
      canopyDepth
    )

    const tx = new Transaction().add(allocTreeIx)

    const txSignature = await sendAndConfirmTransaction(
      connection,
      tx,
      [wallet.payer, merkleTree],
      {
        commitment: "confirmed",
      }
    )
    console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)
  })

  it("Create Tree", async () => {
    // create tree via CPI
    const txSignature = await program.methods
      .anchorCreateTree(
        maxDepthSizePair.maxDepth,
        maxDepthSizePair.maxBufferSize
      )
      .accounts({
        pda: pda,
        merkleTree: merkleTree.publicKey,
        treeAuthority: treeAuthority,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        bubblegumProgram: BUBBLEGUM_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
      })
      .rpc({ skipPreflight: true })
    console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)

    console.log("before merkle tree ")
    // fetch tree account
    const treeAccount = await ConcurrentMerkleTreeAccount.fromAccountAddress(
      connection,
      merkleTree.publicKey
    )

    console.log("MaxBufferSize", treeAccount.getMaxBufferSize())
    console.log("MaxDepth", treeAccount.getMaxDepth())
    console.log("Tree Authority", treeAccount.getAuthority().toString())

    assert.strictEqual(
      treeAccount.getMaxBufferSize(),
      maxDepthSizePair.maxBufferSize
    )
    assert.strictEqual(treeAccount.getMaxDepth(), maxDepthSizePair.maxDepth)
    assert.isTrue(treeAccount.getAuthority().equals(treeAuthority))
  })

  // it("Mint Compressed NFT to Different Account for failing Test ", async () => {
  //   // mint compressed nft via CPI
  //   const txSignature = await program.methods
  //     .mintCompressedNft()
  //     .accounts({
  //       payer:Testerkeypair.publicKey,
  //       pda: pda,
  //       merkleTree: merkleTree.publicKey,
  //       treeAuthority: treeAuthority,
  //       logWrapper: SPL_NOOP_PROGRAM_ID,
  //       bubblegumSigner: bubblegumSigner,
  //       bubblegumProgram: BUBBLEGUM_PROGRAM_ID,
  //       compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  //       tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,

  //       collectionMint: collectionNft.mintAddress,
  //       collectionMetadata: collectionNft.metadataAddress,
  //       editionAccount: collectionNft.masterEditionAddress,
  //     }).signers([Testerkeypair])
  //     .rpc({ commitment: "confirmed" })
  //   console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)

  //   assetId = await extractAssetId(
  //     connection,
  //     txSignature,
  //     merkleTree.publicKey,
  //     program.programId
  //   )
  // })

  // it("fail Mint not owner", async () => {
  //   console.log("before helius",assetId)
  //   const [assetData, assetProofData] = await Promise.all([
  //     heliusApi("getAsset", { id: assetId.toBase58() }),
  //     heliusApi("getAssetProof", { id: assetId.toBase58() }),
  //   ])
  //   console.log("after helius")
  //   console.log("helius data", assetData)
  //   const { compression, ownership } = assetData
  //   const { proof, root } = assetProofData

  //   const treePublicKey = new PublicKey(compression.tree)
  //   const ownerPublicKey = new PublicKey(ownership.owner)
  //   const delegatePublicKey = ownership.delegate
  //     ? new PublicKey(ownership.delegate)
  //     : ownerPublicKey

  //   console.log("owner publickey", ownerPublicKey)
  //   console.log("ownership delegate", delegatePublicKey)
  //   console.log("data hash",compression.data_hash)
  //   const treeAccount = await ConcurrentMerkleTreeAccount.fromAccountAddress(
  //     connection,
  //     treePublicKey
  //   )
  //   const treeAuthorityFromAsset = treeAccount.getAuthority()
  //   const canopyDepth = treeAccount.getCanopyDepth() || 0

  //   const proofPath: AccountMeta[] = proof
  //     .map((node: string) => ({
  //       pubkey: new PublicKey(node),
  //       isSigner: false,
  //       isWritable: false,
  //     }))
  //     .slice(0, assetProofData.proof.length - (!!canopyDepth ? canopyDepth : 0))
  //   const txSignature = await program.methods.mintIfCreatorNft(
  //     Array.from(new PublicKey(root).toBytes()),
  //     Array.from(new PublicKey(compression.creator_hash.trim()).toBytes()),
  //     new anchor.BN(compression.leaf_id),
  //     compression.leaf_id,
  //     Array.from(new PublicKey(compression.data_hash.trim()).toBytes()),
  //   ).accounts({
  //     payer: Testerkeypair.publicKey,
  //     pda: pda,
  //     treeAuthority: treeAuthorityFromAsset,
  //     merkleTree: merkleTree.publicKey,
  //     bubblegumSigner: bubblegumSigner,
  //     logWrapper: SPL_NOOP_PROGRAM_ID,
  //     compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  //     tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //     collectionMint: collectionNft.mintAddress,
  //     collectionMetadata: collectionNft.metadataAddress,
  //     editionAccount: collectionNft.masterEditionAddress,
  //     creator: pda,
  //     collectionIfMint: collectionNft.mintAddress,
  //     collectionIfMetadata: collectionNft.metadataAddress,
  //     bubblegumProgram:BUBBLEGUM_PROGRAM_ID,
  //     assetId:new PublicKey(assetId)
  //   }).signers([Testerkeypair])
  //   .remainingAccounts(proofPath)
  //     .rpc({ commitment: "confirmed" , skipPreflight:true })

  //    console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)


  // })

  it("Mint Compressed NFT if onList before adding", async () => {

    const initTX = await program.methods.initializeList().accounts({
      payer:wallet.payer.publicKey,
      list:list.publicKey,
    }).signers([wallet.payer,list]).rpc({commitment:"confirmed", skipPreflight: true});
    console.log(`https://explorer.solana.com/tx/${initTX}?cluster=devnet`)


    // mint compressed nft via CPI
    const txSignature = await program.methods.mintCompressedNftIfOnlist()
      .accounts({
        pda: pda,
        merkleTree: merkleTree.publicKey,
        treeAuthority: treeAuthority,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        bubblegumSigner: bubblegumSigner,
        bubblegumProgram: BUBBLEGUM_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,

        collectionMint: collectionNft.mintAddress,
        collectionMetadata: collectionNft.metadataAddress,
        editionAccount: collectionNft.masterEditionAddress,
        list:list.publicKey
      })
      .rpc({ commitment: "confirmed" })
    console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)

    assetId = await extractAssetId(
      connection,
      txSignature,
      merkleTree.publicKey,
      program.programId
    )
  })


  it("Add to onchain list",async ()=>{

    const txSignature = await program.methods.addOnChainList().accounts({
      list:list.publicKey
    }).rpc({commitment:"confirmed", skipPreflight: true})

    console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)
  })



  it("Mint Compressed NFT if onList", async () => {
    // mint compressed nft via CPI
    const txSignature = await program.methods.mintCompressedNftIfOnlist()
      .accounts({
        pda: pda,
        merkleTree: merkleTree.publicKey,
        treeAuthority: treeAuthority,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        bubblegumSigner: bubblegumSigner,
        bubblegumProgram: BUBBLEGUM_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,

        collectionMint: collectionNft.mintAddress,
        collectionMetadata: collectionNft.metadataAddress,
        editionAccount: collectionNft.masterEditionAddress,
        list:list.publicKey
      })
      .rpc({ commitment: "confirmed" })
    console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)

    assetId = await extractAssetId(
      connection,
      txSignature,
      merkleTree.publicKey,
      program.programId
    )
  })

 
  it("Mint verify creator", async () => {
    setTimeout(()=>{},500000)
    console.log("before helius",assetId)
    const [assetData, assetProofData] = await Promise.all([
      heliusApi("getAsset", { id: assetId.toBase58() }),
      heliusApi("getAssetProof", { id: assetId.toBase58() }),
    ])
    console.log("after helius")
    console.log("helius data", assetData,assetProofData)
    const { compression, ownership } = assetData
    const { proof, root } = assetProofData
    console.log("proof",proof)
    console.log("root",root)
    const treePublicKey = new PublicKey(compression.tree)
    const ownerPublicKey = new PublicKey(ownership.owner)
    const delegatePublicKey = ownership.delegate
      ? new PublicKey(ownership.delegate)
      : ownerPublicKey

    console.log("owner publickey", ownerPublicKey)
    console.log("ownership delegate", delegatePublicKey)
    const treeAccount = await ConcurrentMerkleTreeAccount.fromAccountAddress(
      connection,
      treePublicKey
    )
    const treeAuthorityFromAsset = treeAccount.getAuthority()
    const canopyDepth = treeAccount.getCanopyDepth() || 0

    const proofPath: AccountMeta[] = proof
      .map((node: string) => ({
        pubkey: new PublicKey(node),
        isSigner: false,
        isWritable: false,
      }))
      .slice(0, assetProofData.proof.length - (!!canopyDepth ? canopyDepth : 0))
      const txSignature = await program.methods.mintIfCreatorNft(
      Array.from(new PublicKey(root.trim()).toBytes()),
      Array.from(new PublicKey(compression.creator_hash.trim()).toBytes()),
      new anchor.BN(compression.leaf_id),
      compression.leaf_id,
      Array.from(new PublicKey(compression.data_hash.trim()).toBytes()),
    ).accounts({
      payer: wallet.payer.publicKey,
      treeAuthority: treeAuthorityFromAsset,
      merkleTree: treePublicKey,
      bubblegumSigner: bubblegumSigner,
      logWrapper: SPL_NOOP_PROGRAM_ID,
      compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      editionAccount: collectionNft.masterEditionAddress,
      creator: pda,
      collectionIfMint: collectionNft.mintAddress,
      collectionIfMetadata: collectionNft.metadataAddress,
      bubblegumProgram:BUBBLEGUM_PROGRAM_ID,
      assetId:new PublicKey(assetId)
    })
    .remainingAccounts(proofPath)
      .rpc({ commitment: "confirmed" , skipPreflight:true})

     console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`)


  })


  

})
