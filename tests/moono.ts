import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { createMint } from "@solana/spl-token";
import { Moono } from "../target/types/moono";

describe("moono", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.moono as Program<Moono>;

  it("ping", async () => {
    const sig = await program.methods.ping().rpc();
    console.log("tx:", sig);
  });

  it("initialize_protocol", async () => {
    const [protocolPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      program.programId
    );

    const tx = await program.methods
      .initializeProtocol()
      .accounts({
        protocol: protocolPda,
        authority: provider.wallet.publicKey,
        system_program: anchor.web3.SystemProgram.programId
      })
      .rpc();

    console.log("tx:", tx);

    const protocolAccount = await program.account.protocolConfig.fetch(protocolPda);

    console.log("protocol:", protocolAccount);

    if (!protocolAccount.authority.equals(provider.wallet.publicKey)) {
      throw new Error("Authority mismatch");
    }

    if (protocolAccount.paused !== false) {
      throw new Error("Paused should be false");
    }
  });

  it("initialize_asset_pool", async () => {
    const [protocolPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      program.programId
    );

    const mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    const [assetPoolPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("asset_pool"), mint.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initializeAssetPool()
      .accounts({
        protocol: protocolPda,
        assetPool: assetPoolPda,
        mint,
        authority: provider.wallet.publicKey,
        system_program: anchor.web3.SystemProgram.programId
      })
      .rpc();

    console.log("tx:", tx);

    const assetPool = await program.account.assetPool.fetch(assetPoolPda);

    console.log("asset pool:", assetPool);

    if (!assetPool.protocol.equals(protocolPda)) {
      throw new Error("Protocol mismatch");
    }

    if (!assetPool.mint.equals(mint)) {
      throw new Error("Mint mismatch");
    }

    if (assetPool.isEnabled !== true) {
      throw new Error("Asset pool should be enabled");
    }

    if (assetPool.allowDeposits !== true) {
      throw new Error("Deposits should be enabled");
    }

    if (assetPool.allowBorrows !== true) {
      throw new Error("Borrows should be enabled");
    }

    if (assetPool.decimals !== 6) {
      throw new Error("Decimals mismatch");
    }
  });

  it("set_asset_pool_flags", async () => {
    const [protocolPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      program.programId
    );

    const mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    const [assetPoolPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("asset_pool"), mint.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeAssetPool()
      .accounts({
        protocol: protocolPda,
        assetPool: assetPoolPda,
        mint,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const tx = await program.methods
      .setAssetPoolFlags(false, false, true)
      .accounts({
        protocol: protocolPda,
        assetPool: assetPoolPda,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log("tx:", tx);

    const assetPool = await program.account.assetPool.fetch(assetPoolPda);

    console.log("updated asset pool:", assetPool);

    if (assetPool.isEnabled !== false) {
      throw new Error("isEnabled should be false");
    }

    if (assetPool.allowDeposits !== false) {
      throw new Error("allowDeposits should be false");
    }

    if (assetPool.allowBorrows !== true) {
      throw new Error("allowBorrows should be true");
    }
  });
});
