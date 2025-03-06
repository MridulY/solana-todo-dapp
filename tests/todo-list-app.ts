import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoListApp } from "../target/types/todo_list_app";
import { assert } from "chai";

describe("todo-list-app", () => {
  // Configure the client to use the local Solana cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Get the program instance from the workspace.
  const program = anchor.workspace.TodoListApp as Program<TodoListApp>;

  // Get the author (wallet) who will sign transactions.
  const author = program.provider as anchor.AnchorProvider;

  it("can create a task", async () => {
    // Generate a new keypair for the task account.
    const task = anchor.web3.Keypair.generate();

    // Call the smart contract to create a new task.
    const tx = await program.methods
      .addingTask("You are awesome") // Task description.
      .accounts({
        task: task.publicKey, // Assign the newly generated task keypair.
        systemProgram: anchor.web3.SystemProgram.programId, // System Program needed for initialization.
      })
      .signers([task]) // Sign the transaction with the task keypair.
      .rpc();

    console.log("Your transaction signature", tx);

    // Fetch the newly created task from the blockchain.
    const taskAccount = await program.account.task.fetch(task.publicKey);
    console.log("Your task", taskAccount);

    // Validate that the task was created correctly.
    assert.equal(
      taskAccount.author.toBase58(),
      author.wallet.publicKey.toBase58(),
      "Task author should match the wallet address"
    );
    assert.equal(taskAccount.text, "You are awesome", "Task text should match");
    assert.equal(
      taskAccount.isDone,
      false,
      "Task should be initially not done"
    );
    assert.ok(taskAccount.createdAt, "Task should have a creation timestamp");
    assert.ok(taskAccount.updatedAt, "Task should have an update timestamp");
  });

  it("can update a task's completion status", async () => {
    // Generate a new task keypair.
    const task = anchor.web3.Keypair.generate();

    // Create a new task before updating it.
    await program.methods
      .addingTask("Complete this important task")
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task, author.wallet.payer])
      .rpc();

    // Update the task status to "done".
    await program.methods
      .updatingTask(true) // Setting is_done = true.
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
      })
      .rpc();

    // Fetch the updated task from the blockchain.
    const taskAccount = await program.account.task.fetch(task.publicKey);
    assert.equal(taskAccount.isDone, true, "Task should be marked as done");
  });

  it("can toggle a task's completion status", async () => {
    // Generate a new keypair for the task.
    const task = anchor.web3.Keypair.generate();

    // Create a task initially marked as not done.
    await program.methods
      .addingTask("Another important task")
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task, author.wallet.payer])
      .rpc();

    // Toggle the completion status to true.
    await program.methods
      .toggleCompletionStatus()
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
      })
      .rpc();

    // Fetch the task after toggling.
    const taskAccount = await program.account.task.fetch(task.publicKey);
    assert.equal(taskAccount.isDone, true, "Task should be toggled to done");

    // Toggle the task status back to false.
    await program.methods
      .toggleCompletionStatus()
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
      })
      .rpc();

    // Fetch the task again.
    const toggledBackTask = await program.account.task.fetch(task.publicKey);
    assert.equal(
      toggledBackTask.isDone,
      false,
      "Task should be toggled back to not done"
    );
  });
});
