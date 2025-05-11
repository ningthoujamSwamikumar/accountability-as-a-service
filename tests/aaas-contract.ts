import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AaasContract } from "../target/types/aaas_contract";
import { BN } from "bn.js";
import { PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";
import { createMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("aaas-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.aaasContract as Program<AaasContract>;
  const pool_id = new BN(1);
  const creator = anchor.getProvider().wallet;
  const feeTokenMint = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  const feeTokenProgram = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

  it("create pool", async () => {
    const tokenMint = await createMint(
      anchor.getProvider().connection,
      anchor.getProvider().wallet.payer,
      anchor.getProvider().wallet.publicKey,
      null,
      6
    );
    // Add your test here.
    const txn = await program.methods.createPool(
      pool_id,
      new BN(1746803391),
      new BN(1747215591),
      300,
      ["Screenshot of an app", "Screenshot of smart watch", "Video recordings"],
      "walk 1000 steps a day",
    )
      .accounts({creator: creator.publicKey, feeTokenMint: tokenMint, feeTokenProgram: TOKEN_PROGRAM_ID})
      .signers([creator.payer])
      .rpc();

    //const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("pool"), pool_id.toArrayLike(Buffer, 'le', 8)], program.programId);

    const pool = (await program.account.pool.all())[0].account;

    expect(pool.poolId == pool_id, "Pool id matched");
    assert(pool.members.length == 0, "No members initially");
  });

  it("join pool", async ()=>{
    //get or create a token account (preferably an ATA)
    
    program.methods.joinPool(pool_id).accounts({})
  })
});
