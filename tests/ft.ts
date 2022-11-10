import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Ft } from "../target/types/ft";

describe("ft", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Ft as Program<Ft>;

  it("Is initialized!", async () => {
    // Add your test here.
   //console.log("Your transaction signature", tx);
  });
});
