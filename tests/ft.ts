import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as spl from '@solana/spl-token';
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { LAMPORTS_PER_SOL, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { Ft } from "../target/types/ft";
import { SplTokenAccountsCoder } from "@project-serum/anchor/dist/cjs/coder/spl-token/accounts";

describe("ft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ft as Program<Ft>;

  let payer = (provider.wallet as NodeWallet).payer;

  let tokenMint: spl.Token;

  let user_sending = provider.wallet.publicKey;

  let vault_account = anchor.web3.Keypair.generate();

  let user_receiving = anchor.web3.Keypair.generate();

  let vault: anchor.web3.PublicKey;

  before(async() => {

    let mint = await spl.Token.createMint(
    provider.connection,
    payer,
    provider.wallet.publicKey,
    provider.wallet.publicKey,
    7,
    spl.TOKEN_PROGRAM_ID,
  );

      //sender's token account
    let sender_token_account = await mint.createAccount(user_sending);
    await mint.mintTo(sender_token_account, payer, [], 10_000_000);//if we can be rich, why not?

    //vault token account
    let vault_token_account = await mint.createAccount(vault_account.publicKey)

    it("Initializes Fund Transfer", async () => {
      //airdrop tokens to payer
      await provider.connection.requestAirdrop(
        payer.publicKey, 1 * LAMPORTS_PER_SOL
      )
    });

  });

  
});
