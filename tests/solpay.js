const anchor = require("@project-serum/anchor");
const assert = require("assert");

describe("solpay", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solpay;

  it("Is initialized!", async () => {
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });

  it("Creates a stream", async () => {
    // Add test for creating a stream
  });

  it("Withdraws from a stream", async () => {
    // Add test for withdrawing from a stream
  });

  // Add more tests as needed
});