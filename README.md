# **IC-PayX - an x402 Payment Framework**

### *EIP-2612 + Multi-Chain Payment + IC-PayX Scan MVP*

**Category:** Decentralized Multi-Asset Payment Infrastructure
**Platform:** DFINITY Internet Computer (ICP)

---

## **1. Executive Summary**

The **IC-PayX x402 Payment Framework** is a modular, decentralized, and keyless payment protocol built for the **Internet Computer (ICP)**.
Based on the **x402 Open Payment Standard**, IC-PayX extends its model with ICP-native capabilities such as **EIP-2612 permits**, **certified data**, and **ECDSA-based cross-chain transactions**.

IC-PayX introduces a unified standard for **multi-asset payments**, **permit-based authorization**, and **decentralized service discovery**, enabling trustless, auditable, and composable payment flows across ICP and Ethereum ecosystems.

### **Key Innovations**

- **ICRC-30: EIP-2612 Permit Extension** Propose a new standard for ICP tokens
- **Multi-Chain Payment Support:** ICP ↔ Ethereum (USDC / ERC20)
- **Internet Identity Integration:** Keyless user authentication
- **Facilitator Registry:** Open and verifiable network of payment facilitators
- **IC-PayX Scan Platform:** Discovery and analytics hub for facilitators and services
- **ECDSA Cross-Chain Signing:** Enables ICP to execute Ethereum transactions

---

## **2. Objectives**

1. Develop a **multi-asset, trustless payment framework** supporting ICRC1/2 and ERC20 assets.
2. Implement **ICRC-30**, an open permit-based authorization standard for ICP tokens.
3. Deliver **cross-chain payment capability** using ICP ECDSA signatures.
4. Launch the **IC-PayX Scan Platform** for service discovery, analytics, and registration.
5. Establish a foundation for **decentralized AI, API, and data commerce** on ICP.

---

## **3. Core Technical Advantages**

### **3.1 Native Decentralization**

All payment modules are implemented as **independent ICP canisters**, each maintaining its own autonomous state and verifiable logic.
This architecture ensures **full transparency**, **auditability**, and **modular composability** across the entire payment network.

---

### **3.2 Keyless Authorization**

Through the **ICRC-30: EIP-2612 Permit Extension** proposal, IC-PayX enables **off-chain signature-based approvals**.
Users authorize payments without ever exposing private keys or maintaining traditional wallets, resulting in **secure, seamless, and frictionless authorization** across dApps and services.

---

### **3.3 Multi-Asset & Cross-Chain Support**

IC-PayX natively supports both **ICRC1/2 (ICP-native tokens)** and **EVM ERC20 assets**, enabling payments across ecosystems.
Using ICP’s built-in **ECDSA signature service**, the framework can **securely sign and broadcast Ethereum transactions**, bridging ICP with the wider Web3 financial landscape.

---

### **3.4 Certified Trust Layer**

Every payment log, facilitator record, and authorization event is stored using **ICP Certified Variables**.
This provides **Merkle-proof verifiability** for frontends and third-party indexers — ensuring that all user-facing data can be validated **without trusting any centralized server**.

---

### **3.5 High Performance**

The Internet Computer’s **Threshold Relay + Chain Key** cryptography achieves **deterministic finality** and **sub-second execution**, making IC-PayX suitable for **real-time microtransactions**, **high-frequency trading**, and **automated service payments**.

---

### **3.6 Zero-Fee User Experience**

Unlike traditional blockchains, the **Internet Computer imposes no gas fees on end users**.
All computation and storage costs are handled by canisters through **cycle-based economics**, meaning:

- End users experience **instant, gasless transactions**, ideal for Web2-like onboarding.
- Developers can design **subscription-based or sponsored-fee models** without compromising decentralization.
- Facilitators and dApps can **abstract transaction costs** entirely, creating a **smooth, scalable payment UX** that’s impossible on gas-metered chains.

This zero-fee model transforms IC-PayX into a **truly frictionless payment network**, accessible to both crypto-native users and mainstream audiences.

---

### 3.6 Unified Technology Stack

| Layer | Technology | Description |
| --- | --- | --- |
| Smart Contracts | Rust (ic-cdk) | Core logic & canister orchestration |
| Identity | Internet Identity | Principal-based authentication |
| Tokens | ICRC1 / ICRC2 / ERC20 (EIP-2612) | Multi-chain assets |
| Authorization | ICRC-30 | Universal permit-based authorization |
| Cross-Chain | ICP ECDSA Service | Ethereum transaction signing |
| Frontend | Next.js / React + agent-js | dApp and Scan UI |
| APIs | GraphQL / REST | Indexer and analytics access |
| Certification | ic-certified-map | Verifiable state proofs |

---

## **4. System Architecture**

```
+-------------------------------------------------------------+
|                        Frontend (dApp)                      |
|  - Payment UI & transaction status                          |
|  - Internet Identity + ICRC-30 permit signing               |
|  - Service registration via IC-PayX Scan                   |
+-------------------------------------------------------------+
                  |
                  v
+-------------------------------------------------------------+
|                IIC-PayXScan / API Gateway Layer             |
|  - Facilitator & Service metadata APIs                       |
|  - Token metadata, price feeds, and reputation data          |
|  - GraphQL / REST endpoints for explorers and analytics      |
+-------------------------------------------------------------+
                  |
                  v
+-------------------------------------------------------------------------+
|                        ICP Smart Contract Layer                         |
|                                                                         |
|  +------------------+   +------------------+   +----------------------+ |
|  | Payment Manager  |   | Authorization    |   | Facilitator Registry | |
|  | - Orders & exec. |   | - ICRC-30 permits|   | - Registration & DAO | |
|  +------------------+   +------------------+   +----------------------+ |
|                                                                         |
|  +------------------+   +-------------------+                           |
|  | Asset Manager    |   | Transaction Logger|                           |
|  | - Multi-asset    |   | - Certified logs  |                           |
|  +------------------+   +-------------------+                           |
|                                                                         |
|  +------------------------------------------+                           |
|  | ICRC-30 Permit Transfer                  |                           |
|  | - Permit verification & cross-token exec |                           |
|  | - Fallback approval flow                 |                           |
|  +------------------------------------------+                           |
+-------------------------------------------------------------------------+
                  |
                  v
+-------------------------------------------------------------+
|                External Asset Interfaces                    |
|  - ICRC Tokens(ICRC-30)                                     |
|  - ERC20 (EIP-2612) via Ethereum Gateway                    |
|  - Third-party payment bridges                              |
+-------------------------------------------------------------+
```

### Architectural Principles

- **Composable:** Each module is an autonomous canister.
- **Auditable:** Certified Merkle-proof verification.
- **Keyless:** ICRC-30 replaces manual approvals.
- **Cross-Chain:** ECDSA signing supports external networks.
- **Discoverable:** IC-PayX Scan provides registry & analytics.

---

## **5. Core Modules**

| Module | Function | Highlights |
| --- | --- | --- |
| **Payment Manager** | Executes payment orders | ICRC-30 permits, fallback transfers, order batching |
| **Authorization (ICRC-30)** | Manages scope-based approvals | Compatible with EIP-2612, supports expiry and replay protection |
| **ICRC-30 Permit Transfer** | Cross-token verification | Unified flow for ICP + ETH assets |
| **Asset Manager** | Multi-asset custody | ICRC, ERC20, pay & transfer support |
| **Transaction Logger** | Certified record-keeping | Tamper-proof via certified variables |
| **Facilitator Registry** | Decentralized node registry | Metadata, discovery APIs, scoring engine |
| **Risk & Security** | Fraud & abuse protection | Rate limits, ACLs, optional multi-sig |
| **IC-PayX Scan Layer** | Service discovery portal | Frontend for registration and analytics |

---

## **6. X402 Protocol Integration**

The **x402 protocol** defines open, secure interactions among **Buyers**, **Facilitators**, **Sellers**, and the **Discovery Network**.
IC-PayX adopts and extends this standard for ICP, ensuring transparent, decentralized payment routing.

### 6.1 Core Roles

| Actor | Role | Description |
| --- | --- | --- |
| **Buyer** | Purchaser | Initiates payment via signed ICRC-30 intent |
| **Facilitator** | Mediator | Executes payments, issues certified receipts |
| **Seller** | Provider | Registers services, validates receipts |
| **Discovery** | Index | Lists and verifies registered facilitators/services |

### 6.2 Transaction Flow

```
Buyer ──► Facilitator ──► Seller-------
   ▲           │                       │
   │           ▼                       │
   └──── Discovery (IC-PayX Scan) ◄───┘
```

1. Buyer authenticates via **Internet Identity**, generates **ICRC-30 permit**.
2. Facilitator validates permit, executes payment via **Payment Manager**.
3. Seller confirms via **certified transaction logs**.
4. Discovery (IC-PayX Scan) maintains certified metadata and analytics.

### 6.3 Why X402?

- **Open Participation:** Any developer can run a Facilitator node.
- **No Vendor Lock-in:** Market-driven facilitator competition.
- **Composability:** Buyers, Sellers, and Facilitators interoperate via open APIs.
- **Verifiability:** Every payment path is cryptographically auditable.

### 6.4 IC-PayX Enhancements

| Feature | Description |
| --- | --- |
| **ICRC-30 Authorization Layer** | Unifies ICP and EVM permit standards |
| **Certified Receipts** | Verifiable Merkle-based payment proofs |
| **IC-PayX Scan API** | Search by token, fee, uptime, or region |
| **Cross-Chain Settlement** | ICP executes ETH transactions via ECDSA |
| **Trust Scoring Model** | Facilitator ratings based on performance metrics |

---

## **7. IC-PayX Scan Platform**

**IC-PayX Scan** is the discovery, analytics, and registration gateway of the IC-PayX network.

### Core Capabilities

| Feature | Description |
| --- | --- |
| **Facilitator Explorer** | Browse registered facilitators and reputation scores |
| **Service Registry** | On-chain registration for dApps, APIs, and services |
| **Certified Metadata** | ICP-verified facilitator and service info |
| **Analytics Dashboard** | Volume, uptime, and performance insights |
| **Discovery API** | Query by token, fee model, or category |
| **Registration Portal** | Internet Identity-based onboarding for new participants |

---

## **8. 4-Month Development Plan**

| Phase | Duration | Focus | Deliverables |
| --- | --- | --- | --- |
| **M1 — Architecture & Design** | Weeks 1–2 | Define ICRC-30, system schema | ICRC-30 spec, Candid types, Scan API schema |
| **M2 — Core Implementation** | Weeks 3–6 | Build ICP canisters | PaymentManager, Authorization, Logger |
| **M3 — ETH Integration & Scan** | Weeks 7–9 | Enable cross-chain & deploy Scan base | EIP-2612 validation, ECDSA signing, Registry canister |
| **M4 — Frontend & UX** | Weeks 10–12 | Integrate UI & discovery | Internet Identity + ICRC-30 flow, Scan portal |
| **M5 — Test & Launch** | Weeks 13–16 | Test & release | Cross-chain tests, registry flow, documentation |

---

## **9. Expected Deliverables**

- Full implementation of **ICRC-30: EIP-2612 Permit Extension**
- Operational **multi-chain payment MVP** on ICP
- **IC-PayX Scan Platform** (Facilitator & Service discovery)
- Cross-chain signing between ICP and Ethereum
- Certified logging and verifiable payment proofs
- Open-source SDK & documentation

---

## **10. Future Expansion**

| Area | Focus |
| --- | --- |
| **ICRC-30 Standardization** | Submit as open standard to DFINITY Forum |
| **Reputation Engine v2** | Dynamic facilitator scoring |
| **Streaming Payments** | Continuous and subscription settlement |
| **DAO Governance** | Facilitator staking and treasury rewards |
| **Multi-Chain Expansion** | Support for Base, Solana, and L2 networks |

---

## **11. Impact**

- Establishes **ICP’s first cross-chain, keyless payment framework**
- Enables **AI, API, and DePIN** services to monetize directly on-chain
- Bridges ICP and EVM ecosystems through unified authorization
- Sets a new **open standard (ICRC-30)** for secure, verifiable payments

---

## **12. Conclusion**

The **IC-PayX x402 Payment Framework** represents a transformative step in Web3 financial infrastructure.
By merging **x402’s decentralized role model** with **ICRC-30 authorization** and the **IC-PayX Scan Platform**, it enables a truly **open, auditable, and composable** payment ecosystem — where services, facilitators, and users interact without intermediaries or private key exposure.

IC-PayX positions the **Internet Computer** as a central hub for **next-generation cross-chain payment automation** and decentralized commerce.

---
