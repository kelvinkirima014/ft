import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as spl from '@solana/spl-token';
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { LAMPORTS_PER_SOL, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { Ft } from "../target/types/ft";
import { SplTokenAccountsCoder } from "@project-serum/anchor/dist/cjs/coder/spl-token/accounts";

describe("ft", () => {

  //configure the client to use the desired cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  //main wallet to fund transactions
  const payer = provider.wallet as anchor.Wallet;

  //our Rust onchain program IDL
  const program = anchor.workspace.ft as Program<Ft>;

  //keypair that represents our token
  const mintKeypair = anchor.web3.Keypair.generate();

  it("initializes transfer", async() => {
    
  })

});
