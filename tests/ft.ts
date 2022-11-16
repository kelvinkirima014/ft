import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { LAMPORTS_PER_SOL, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { Ft } from "../target/types/ft";

describe("ft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ft as Program<Ft>;

  let payer = (provider.wallet as NodeWallet).payer;

  it("Initialize Fund Transfer", async () => {
    //airdrop tokens to payer
    await provider.connection.requestAirdrop(
      payer.publicKey, 1 * LAMPORTS_PER_SOL
    )
  });
});
