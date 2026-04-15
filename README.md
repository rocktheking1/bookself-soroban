# Stellar Task Tracker DApp

**Stellar Task Tracker DApp** - Blockchain-Based Decentralized Task & To-Do Management System

## Project Description

Stellar Task Tracker DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure, immutable platform for managing daily tasks, lab assignments, and project milestones directly on the blockchain. The contract ensures that your to-do lists are stored transparently and allows you to track the exact completion status of every task without relying on centralized database providers.

The system allows users to create tasks, mark them as completed, retrieve their task lists, and delete them when no longer needed. Each task is uniquely identified and stored within the contract's instance storage, ensuring reliable data persistence and seamless tracking of your progress.

## Project Vision

Our vision is to revolutionize personal and collaborative productivity in the digital age by:

- **Decentralizing Productivity**: Moving task management from centralized servers to a global, distributed blockchain.
- **Ensuring Accountability**: Empowering users to have complete control and a permanent record of their completed tasks and milestones.
- **Guaranteeing Immutability**: Providing a tamper-proof record of what tasks were added and completed, ensuring transparency in collaborative environments.
- **Building Trustless Systems**: Creating a platform where task tracking is guaranteed by code, making it highly reliable for managing complex project deliverables or shared assignment progress.

We envision a future where task management is truly sovereign, allowing individuals and teams to track their goals with complete autonomy and transparency.

## Key Features

### 1. **Simple Task Creation**
- Create tasks with a single function call.
- Specify a `title` and detailed `description` for each task.
- Automated ID generation and default "incomplete" status assignment.

### 2. **Dynamic Status Tracking**
- Dedicated functionality to mark tasks as completed (`is_completed: true`).
- Maintain the task history rather than simply deleting it, allowing you to review your finished work.

### 3. **Efficient Data Retrieval**
- Fetch all stored tasks in a single call to view your entire backlog and completed items.
- Structured data representation (Boolean flags for completion) for easy frontend UI integration.

### 4. **Secure Deletion & Management**
- Remove specific tasks using their unique IDs to keep your active workspace clean.
- Immediate update of the task list after deletion.

### 5. **Stellar Network Integration**
- Leverages the high speed and low cost of the Stellar network.
- Built using the modern Soroban Smart Contract SDK in Rust.

## Contract Details

- Contract Address: CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M
  ![alt text](screenshot.png)

## Future Scope

### Short-Term Enhancements
1. **Due Dates & Deadlines**: Add timestamp integration to track when a task is due (e.g., for weekly lab reports or project submissions).
2. **Priority Levels**: Introduce flags for High, Medium, and Low priority tasks.
3. **Task Categorization**: Add tags to separate different subjects, projects, or personal chores.

### Medium-Term Development
4. **Collaborative Task Assignment**: Allow tasks to be assigned to specific Stellar public keys so team members can share a single contract to manage group work.
5. **Progress Metrics**: Introduce percentage-based progress tracking instead of just a simple boolean (true/false) completion status.
6. **Notification System**: Off-chain bridge to alert users when a shared task is marked as complete.

### Long-Term Vision
7. **Bounty & Reward System**: Integrate token payouts when specific tasks are marked as completed (useful for DAO bounties or incentivized team projects).
8. **Decentralized UI Hosting**: Host the frontend dashboard on IPFS or similar decentralized platforms.
9. **Cross-Contract Triggers**: Allow task completion to automatically trigger events in other smart contracts.

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using these main functions:

- `create_task()` - Add a new task to your list with a title and description.
- `get_tasks()` - Retrieve all stored tasks and view their current completion status.
- `complete_task()` - Update a specific task's status to completed using its ID.
- `delete_task()` - Permanently remove a specific task by its ID.

---
**Stellar Task Tracker DApp** - Securing Your Productivity on the Blockchain