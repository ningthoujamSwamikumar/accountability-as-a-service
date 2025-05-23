import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AaasContract } from "../target/types/aaas_contract";
import { BN } from "bn.js";
import { PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";

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
      new BN(1747031103),
      new BN(1747215591),
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
});
