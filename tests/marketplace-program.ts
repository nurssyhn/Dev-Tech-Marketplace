import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MarketplaceProgram } from "../target/types/marketplace_program";
import { randomBytes } from "crypto";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
describe("marketplace-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .MarketplaceProgram as Program<MarketplaceProgram>;

  const provider = anchor.getProvider();
  const connection = provider.connection;

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string) => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`
    );
    return signature;
  };

  const seed = new anchor.BN(randomBytes(8));
  const employee = Keypair.generate();
  const employer = Keypair.generate();

  // PDAs

  const employeePDA = PublicKey.findProgramAddressSync(
    [Buffer.from("User"), employee.publicKey.toBuffer()],
    program.programId
  )[0];

  const employerPDA = PublicKey.findProgramAddressSync(
    [Buffer.from("User"), employer.publicKey.toBytes()],
    program.programId
  )[0];

  it("Airdrop", async () => {
    let airdropIx = await Promise.all(
      [employee, employer].map(async (k) => {
        return await anchor
          .getProvider()
          .connection.requestAirdrop(k.publicKey, 10 * LAMPORTS_PER_SOL)
          .then(confirm);
      })
    );

    console.log("Airdrop complete", airdropIx);
  });

  it("Is initialized!", async () => {
    // Add your test here.

    const name = "Nikhil";
    const email = "test@gmail.com";
    const profileImage = "image_url";
    const skills = ["React", "Typescript"];
    const tx = await program.methods
      .initializeEmployeeProfile(name, email, profileImage, skills)
      .accounts({
        user: employee.publicKey,
        userProfile: employeePDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([employee])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });
});
