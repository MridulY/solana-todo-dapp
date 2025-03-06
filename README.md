# 📝 Solana To-Do List Smart Contract (Dapp)

This is a **decentralized To-Do List Dapp** built on the **Solana blockchain** using the **Anchor framework**.  
It enables users to **create, update, delete, and fetch tasks** stored on-chain.

Each **task is stored as a separate Solana account**, making the system **fully decentralized**.

## ✅ Features

1. **On-Chain Task Storage**  
   - Each task is stored **in a separate Solana account**.
   - No external database is needed.

2. **CRUD Operations (Create, Read, Update, Delete)**
   - Users can **add, fetch, update, and delete** their tasks.

3. **Optimized for Solana’s Rent System**
   - Tasks are **rent-exempt** to persist on-chain.
   - Storage allocation is **predefined to 1693 bytes**.

4. **Access Control**
   - Only **the task creator (wallet owner)** can modify or delete the task.

5. **Efficient Task Retrieval**
   - Fetch **all tasks for a wallet** or **a specific task by ID**.

6. **Anchor Framework for Simplified Development**
   - Reduces **boilerplate code** and **ensures security**.

## 🏗 Smart Contract Structure

The contract has the following **programs and accounts**:

1. **Functions:**
   - `adding_task` → Create a new task
   - `updating_task` → Update a task
   - `deleting_task` → Delete a task
   - `toggle_completion_status` → Mark task as complete/incomplete

2. **Accounts:**
   - `Task` → Stores task details
   - `Author` → The user's wallet that created the task
   - `SystemProgram` → Used for Solana transactions

3. **Error Handling:**
   - `TextTooLong` → If task text exceeds 400 characters
   - `Unauthorized` → If a user tries to update/delete another user’s task
  

---

## **🛠 6️⃣ Design Decisions**
```md
## 🛠 Design Decisions

1. Solana Smart Contract with Anchor**
   - Anchor framework simplifies smart contract development.
   - Automatically handles **security checks and account validation**.

2. Each Task is a Separate Solana Account**
   - Allows users to store multiple tasks without a centralized database**.
   - Ensures **full ownership and decentralization**.

3. Fixed Storage Allocation (`Task::LEN = 1693` bytes)**
   - Pre-allocated storage ensures efficient data retrieval**.
   - Prevents **serialization issues**.

4. Task Ownership Verification**
   - Users cannot modify or delete** tasks they do not own.
   - Ensures **data integrity**.

5. Stateless Functions for Efficient Execution**
   - The contract does **not store program state, reducing execution costs.
   - Data is fetched **only when needed**.


