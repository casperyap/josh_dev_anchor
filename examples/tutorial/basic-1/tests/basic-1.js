const assert = require("assert");
const anchor = require("@coral-xyz/anchor");
const { SystemProgram } = anchor.web3;

describe("basic-1", () => {
  // Use a local provider.
  const provider = anchor.AnchorProvider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // Create a global variable to store Account
  let _myAccount = null;

  // Create a global variable to store the set value
  let in_value = 201;

  it("Creates and initializes an account in a single atomic transaction (simplified)", async () => {
    // #region code-simplified
    // The program to execute.
    const program = anchor.workspace.Basic1;

    // The Account to create.
    const myAccount = anchor.web3.Keypair.generate();

    // Create the new account and initialize it with the program.
    // #region code-simplified
    
    // // Josh's DevBox Youtube code.
    // await program.rpc.initialize({
    //   accounts: {
    //     myAccount: myAccount.publicKey,
    //     user: provider.wallet.publicKey,
    //     SystemProgram: SystemProgram.programId,
    //   },
    //   signers: [myAccount],
    // });

    //Default generated code.
    await program.methods
      .initialize() //new anchor.BN(1234)
      .accounts({
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([myAccount])
      .rpc();
    // #endregion code-simplified

    // Fetch the newly created account from the cluster.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was initialized.
    // assert.ok(account.data.eq(new anchor.BN(1234)));
    assert.equal(account.data, 0);

    // Store the account for the next test.
    _myAccount = myAccount;
  });

  it("Updates a previously created account", async () => {
    const myAccount = _myAccount;

    // #region update-test

    // The program to execute.
    const program = anchor.workspace.Basic1;

    // Invoke the update rpc.
    await program.methods
      .set(new anchor.BN(in_value))
      .accounts({
        myAccount: myAccount.publicKey,
      })
      .rpc();

    // Fetch the newly updated account.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was mutated.
    assert.ok(account.data, in_value);

    // #endregion update-test
  });

  it("Increment", async () => {
    const myAccount = _myAccount;

    // #region update-test

    // The program to execute.
    const program = anchor.workspace.Basic1;

    // Invoke the update rpc.
    await program.methods
      .increment()
      .accounts({
        myAccount: myAccount.publicKey,
      })
      .rpc();

    // Fetch the newly updated account.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was mutated.
    assert.ok(account.data, in_value + 1);

    // #endregion update-test
  });
  
  it("Decrement", async () => {
    const myAccount = _myAccount;

    // #region update-test

    // The program to execute.
    const program = anchor.workspace.Basic1;

    // Invoke the update rpc.
    await program.methods
      .decrement()
      .accounts({
        myAccount: myAccount.publicKey,
      })
      .rpc();

    // Fetch the newly updated account.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was mutated.
    assert.ok(account.data, in_value);

    // #endregion update-test
  });
});
