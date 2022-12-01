import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BettingGame } from "../target/types/betting_game";

describe("BettingGame", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BettingGame as Program<BettingGame>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
