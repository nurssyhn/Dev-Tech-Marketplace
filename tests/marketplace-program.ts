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

  const solAmount = 10 * LAMPORTS_PER_SOL;

  const seed = new anchor.BN(randomBytes(8));
  const employee = Keypair.generate();
  const employee2 = Keypair.generate();
  const employer = Keypair.generate();
  const index = 1;
  // PDAs

  const employeePDA = PublicKey.findProgramAddressSync(
    [Buffer.from("User"), employee.publicKey.toBuffer()],
    program.programId
  )[0];

  const employee2PDA = PublicKey.findProgramAddressSync(
    [Buffer.from("User"), employee2.publicKey.toBuffer()],
    program.programId
  )[0];

  const employerPDA = PublicKey.findProgramAddressSync(
    [Buffer.from("User"), employer.publicKey.toBuffer()],
    program.programId
  )[0];

  const jobPDA = PublicKey.findProgramAddressSync(
    [Buffer.from("Job"), employer.publicKey.toBuffer()],
    program.programId
  )[0];

  const escrow = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), employer.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), escrow.toBuffer()],
    program.programId
  )[0];

  it("Airdrop", async () => {
    let airdropIx = await Promise.all(
      [employee, employer, employee2].map(async (k) => {
        return await anchor
          .getProvider()
          .connection.requestAirdrop(k.publicKey, 20 * LAMPORTS_PER_SOL)
          .then(confirm);
      })
    );

    console.log("Airdrop complete", airdropIx);
  });

  it("Employee profile initialized!", async () => {
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

  it("Employee2 profile initialized!", async () => {
    // Add your test here.

    const name = "nick";
    const email = "nick@gmail.com";
    const profileImage = "image_url";
    const skills = ["React", "Node", "rust"];
    const tx = await program.methods
      .initializeEmployeeProfile(name, email, profileImage, skills)
      .accounts({
        user: employee2.publicKey,
        userProfile: employee2PDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([employee2])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });

  it("Employer Profile initialized!", async () => {
    // Add your test here.

    const name = "Nikhil";
    const email = "test@gmail.com";
    const profileImage = "image_url";
    const tx = await program.methods
      .initializeEmployerProfile(name, email, profileImage)
      .accounts({
        user: employer.publicKey,
        userProfile: employerPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([employer])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });

  it("New Job initialized!", async () => {
    // Add your test here.

    const job_title = "Frontend Developer";
    const job_description = "With 1+ YOE";
    const tags = "React,Nextjs,Typescript";
    const amount = new anchor.BN(solAmount);
    const tx = await program.methods
      .initializeNewJob(jobPDA, job_title, job_description, tags, amount)
      .accounts({
        owner: employer.publicKey,
        job: jobPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([employer])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });

  it("Job application!", async () => {
    // Add your test here.

    const tx = await program.methods
      .applyForJob()
      .accounts({
        user: employee.publicKey,
        job: jobPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([employee])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });
  it("Job application! - 2", async () => {
    // Add your test here.

    const tx = await program.methods
      .applyForJob()
      .accounts({
        user: employee2.publicKey,
        job: jobPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([employee2])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });

  it("Accept Job Application ", async () => {
    // Add your test here.

    const tx = await program.methods
      .acceptJobApplication(index, seed)
      .accounts({
        owner: employer.publicKey,
        job: jobPDA,
        escrow,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([employer])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });

  it(" Job Completion Update ", async () => {
    // Add your test here.

    const tx = await program.methods
      .updateJobCompletion()
      .accounts({
        owner: employer.publicKey,
        job: jobPDA,
        user: employee2.publicKey,
        escrow,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([employer])
      .rpc({ skipPreflight: true })
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });
});
