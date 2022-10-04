import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { GoPhish } from "../target/types/go_phish";

export function loadWalletKey(keypairFile:string): anchor.web3.Keypair {
  if (!keypairFile || keypairFile == '') {
    throw new Error('Keypair is required!');
  }
  const fs = require("fs");
  const loaded = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(keypairFile).toString())),
  );
  return loaded;
}

describe("go_phish", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GoPhish as Program<GoPhish>;

  const myKey = loadWalletKey("/Users/Temitope/.config/solana/id.json")

  const seed1 = Buffer.from(anchor.utils.bytes.utf8.encode("randomness"));
  const seed2 = Buffer.from(myKey.publicKey.toBytes())
  const [phishPDA, _bump] = findProgramAddressSync([seed1, seed2], program.programId);

  it("Is adding to phish list!", async () => {
    // Add your test here.
    // const tx = await program.methods.init().accounts({
    //   owner: myKey.publicKey,
    //   phish: phishPDA,
    //   systemProgram: anchor.web3.SystemProgram.programId
    // }).rpc();

    const tx = await program.methods.goPhish("google~com").accounts({
      phish: phishPDA,
      owner: myKey.publicKey
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
