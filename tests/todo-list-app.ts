import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoListApp } from "../target/types/todo_list_app";
import { assert } from "chai";

describe("todo-list-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.TodoListApp as Program<TodoListApp>;
  const author = program.provider as anchor.AnchorProvider;
  it("can create a task", async () => {
    const task = anchor.web3.Keypair.generate();
    const tx = await program.methods
      .addingTask("You are awesome")
      .accounts({
        task: task.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task])
      .rpc();
    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.fetch(task.publicKey);
    console.log("Your task", taskAccount);

    assert.equal(
      taskAccount.author.toBase58(),
      author.wallet.publicKey.toBase58()
    );
    assert.equal(taskAccount.text, "You are awesome");
    assert.equal(taskAccount.isDone, false);
    assert.ok(taskAccount.createdAt);
    assert.ok(taskAccount.updatedAt);
  });

  it("can update a task's completion status", async () => {
    const task = anchor.web3.Keypair.generate();

    // Create a task initially marked as not done
    await program.methods
      .addingTask("Complete this important task")
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task, author.wallet.payer])
      .rpc();

    // Update the task to be marked as done
    await program.methods
      .updatingTask(true)
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
      })
      .rpc();

    // Fetch the updated task
    const taskAccount = await program.account.task.fetch(task.publicKey);
    assert.equal(taskAccount.isDone, true, "Task should be marked as done");
  });

  it("can toggle a task's completion status", async () => {
    const task = anchor.web3.Keypair.generate();

    // Create a new task initially marked as not done
    await program.methods
      .addingTask("Another important task")
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task, author.wallet.payer])
      .rpc();

    // Toggle the completion status to true
    await program.methods
      .toggleCompletionStatus()
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
      })
      .rpc();

    // Fetch the toggled task
    const taskAccount = await program.account.task.fetch(task.publicKey);
    assert.equal(taskAccount.isDone, true, "Task should be toggled to done");

    // Toggle back to false
    await program.methods
      .toggleCompletionStatus()
      .accounts({
        task: task.publicKey,
        author: author.wallet.publicKey,
      })
      .rpc();

    // Refetch the task
    const toggledBackTask = await program.account.task.fetch(task.publicKey);
    assert.equal(
      toggledBackTask.isDone,
      false,
      "Task should be toggled back to not done"
    );
  });



});