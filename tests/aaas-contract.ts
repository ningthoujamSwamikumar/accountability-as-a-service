import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AaasContract } from "../target/types/aaas_contract";
import { BN } from "bn.js";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";
import { createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { airdropIfRequired } from "@solana-developers/helpers";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("aaas-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.aaasContract as Program<AaasContract>;
  const creator = anchor.getProvider().wallet;

  it("create pool", async () => {
    const pool_id = new BN(1);

    const tokenMint = await createMint(
      anchor.getProvider().connection,
      anchor.getProvider().wallet.payer,
      anchor.getProvider().wallet.publicKey,
      null,
      6
    );
    const txn = await program.methods.createPool(
      pool_id,
      new BN(1746803391),
      new BN(1747215591),
      300,
      ["Screenshot of an app", "Screenshot of smart watch", "Video recordings"],
      "walk 1000 steps a day",
    )
      .accounts({ creator: creator.publicKey, feeTokenMint: tokenMint, feeTokenProgram: TOKEN_PROGRAM_ID })
      .signers([creator.payer])
      .rpc();

    //const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("pool"), pool_id.toArrayLike(Buffer, 'le', 8)], program.programId);

    const pool = (await program.account.pool.all())[0].account;

    expect(pool.poolId == pool_id, "Pool id matched");
    assert(pool.members.length == 0, "No members initially");
  });

  it("join pool", async () => {
    const tokenMint = await createMint(
      anchor.getProvider().connection,
      anchor.getProvider().wallet.payer,
      anchor.getProvider().wallet.publicKey,
      null,
      6
    );

    //create pool
    const pool_id = new BN(2);
    await program.methods.createPool(
      pool_id,
      new BN(1847414977),
      new BN(1857414977),
      300,
      ["Screenshot of an app", "Screenshot of smart watch", "Video recordings"],
      "walk 1000 steps a day",
    )
      .accounts({ creator: creator.publicKey, feeTokenMint: tokenMint, feeTokenProgram: TOKEN_PROGRAM_ID })
      .signers([creator.payer])
      .rpc();

    //get or create a token account (preferably an ATA)
    //lets just consider a ATA for now
    const feeTokenAccount = await getOrCreateAssociatedTokenAccount(
      anchor.getProvider().connection,
      anchor.getProvider().wallet.payer,
      tokenMint,
      anchor.getProvider().wallet.publicKey
    );

    await mintTo(
      anchor.getProvider().connection,
      anchor.getProvider().wallet.payer,
      tokenMint,
      feeTokenAccount.address,
      anchor.getProvider().wallet.payer,
      1000 * 1000000
    );

    const accountInfo = await anchor.getProvider().connection.getParsedAccountInfo(feeTokenAccount.address);
    console.log("Account Info:", accountInfo);

    const txn = await program.methods.joinPool(pool_id)
      .accounts({
        feeTokenAccount: feeTokenAccount.address,
        feeTokenMint: tokenMint,
        feeTokenProgram: TOKEN_PROGRAM_ID,
        joiner: anchor.getProvider().publicKey
      })
      .signers([])
      .rpc();

    const pool = (await program.account.pool.all())[0].account;
    console.log("pool after join pool", pool);
    console.log("join pool transaction", txn);
    assert(pool.members.length > 0, "Pool has 0 member");
    console.log("all acounts", (await program.account.pool.all()));
  })

  it("vote pool", async () => {
    const pool_id = new BN(3);
    const tokenMint = await createMint(
      anchor.getProvider().connection,
      anchor.getProvider().wallet.payer,
      anchor.getProvider().wallet.publicKey,
      null,
      6
    );
    //create pool
    await program.methods.createPool(
      pool_id,
      new BN(1847414977),
      new BN(1857414977),
      300,
      ["Screenshot of an app", "Screenshot of smart watch", "Video recordings"],
      "walk 1000 steps a day",
    )
      .accounts({ creator: creator.publicKey, feeTokenMint: tokenMint, feeTokenProgram: TOKEN_PROGRAM_ID })
      .signers([creator.payer])
      .rpc();

    //add members to the pool
    // console.log("user1 === ", anchor.getProvider().wallet.publicKey.toBase58());
    // const txn1 = await createAccountsAndJoinPool(anchor.getProvider().wallet.payer, tokenMint, pool_id);
    /*
    const user2 = Keypair.generate();
    console.log("user2 === ", user2.publicKey.toBase58());
    await createAccountsAndJoinPool(user2, tokenMint, pool_id);
    const user3 = Keypair.generate();
    console.log("user3 === ", user3.publicKey.toBase58());
    await createAccountsAndJoinPool(user3, tokenMint, pool_id);
    const user4 = Keypair.generate();
    console.log("user4 === ", user4.publicKey.toBase58());
    await createAccountsAndJoinPool(user4, tokenMint, pool_id);
    const user5 = Keypair.generate();
    await createAccountsAndJoinPool(user5, tokenMint, pool_id);
    const user6 = Keypair.generate();
    await createAccountsAndJoinPool(user6, tokenMint, pool_id);
    const user7 = Keypair.generate();
    await createAccountsAndJoinPool(user7, tokenMint, pool_id);
    const user8 = Keypair.generate();
    await createAccountsAndJoinPool(user8, tokenMint, pool_id);
    const user9 = Keypair.generate();
    await createAccountsAndJoinPool(user9, tokenMint, pool_id);
    const user10 = Keypair.generate();
    await createAccountsAndJoinPool(user10, tokenMint, pool_id);
    const user11 = Keypair.generate();
    await createAccountsAndJoinPool(user11, tokenMint, pool_id);
    */

    //add multiple members 
    const joiner1 = Keypair.generate();
    const joiner2 = Keypair.generate();

    // Airdrop SOL to users
    const provider = program.provider;
    await provider.connection.requestAirdrop(joiner1.publicKey, 1e9);
    await provider.connection.requestAirdrop(joiner2.publicKey, 1e9);

    const feeTokenAccount1 = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      joiner1,
      tokenMint,
      joiner1.publicKey
    );
    const feeTokenAccount2 = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      joiner2,
      tokenMint,
      joiner2.publicKey
    );
    await mintTo(
      program.provider.connection,
      joiner2,
      tokenMint,
      feeTokenAccount2.address,
      program.provider.wallet.publicKey,
      1000 * 1000000
    );
    await program.methods
      .joinPool(pool_id)
      .accounts({
        joiner: joiner1.publicKey,
        feeTokenAccount: feeTokenAccount1.address,
        feeTokenMint: tokenMint,
        feeTokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([joiner1])
      .rpc();

    await program.methods
      .joinPool(pool_id)
      .accounts({
        joiner: joiner2.publicKey,
        feeTokenAccount: feeTokenAccount2.address,
        feeTokenMint: tokenMint,
        feeTokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([joiner2])
      .rpc();

    //vote pool 
  });



  const createAccountsAndJoinPool = async (user: anchor.web3.Keypair, tokenMint: PublicKey, pool_id: anchor.BN) => {
    const connection = anchor.getProvider().connection;
    const mintAuthority = anchor.getProvider().wallet.publicKey;
    await airdropIfRequired(connection, user.publicKey, 5 * LAMPORTS_PER_SOL, 0.5 * LAMPORTS_PER_SOL);
    const feeTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      tokenMint,
      user.publicKey
    );
    await mintTo(
      connection,
      user,
      tokenMint,
      feeTokenAccount.address,
      mintAuthority,
      1000 * 1000000
    );

    return await program.methods.joinPool(pool_id)
      .accounts({
        joiner: user.publicKey,
        feeTokenAccount: feeTokenAccount.address,
        feeTokenMint: tokenMint,
        feeTokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([])
      .rpc();
  }
})
