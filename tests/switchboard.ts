import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { Switchboard } from "../target/types/switchboard";
import * as fs from "fs";

const historyBuffer = new anchor.web3.PublicKey(
  "7LLvRhMs73FqcLkA8jvEE1AM2mYZXTmqfUv8GAEurymx"
);
anchor.setProvider(anchor.AnchorProvider.env());
const switchboardProgram = new anchor.web3.PublicKey(
  "Eyx38NvnsWRpQJho1fTzj7BvJB29d4JTJYTrkVUiQ4Ur"
);
const authority = anchor.AnchorProvider.env().wallet as anchor.Wallet;
console.log("Signer:" + authority.publicKey.toBase58());
const rpc = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"));
const idl = JSON.parse(
  fs.readFileSync(
    "/mnt/c/Users/home/PycharmProjects/solana/Switchboard/switchboard/target/idl/switchboard.json",
    "utf8"
  )
);
const feedAccPubkey = new anchor.web3.PublicKey(
  "EWhGFf3wij55h535dfaLzUf7PuWyPog1xQkjYZenkcXa"
);
const provider = new anchor.AnchorProvider(
  rpc,
  authority,
  anchor.AnchorProvider.defaultOptions()
);
const program = new anchor.Program(
  idl,
  switchboardProgram,
  provider
) as anchor.Program<Switchboard>;
// describe("switchboard create acount", async () => {
//   it("Is Created!", async () => {
//     let feedAcc = anchor.web3.Keypair.generate();
//     let sig = await program.methods
//       .createSolFeed()
//       .accounts({
//         feedVectorAcc: feedAcc.publicKey,
//         authority: authority.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([feedAcc])
//       .rpc();
//     console.log("Signature:"+sig);
//     console.log("FeedAccountPubkey:"+feedAcc.publicKey.toBase58());
//   });
// });

// describe("read feed", async () => {
//   it("read feed", async () => {
//     const days = new anchor.BN(365);
//     const period = new anchor.BN(3600);
//     const sig = await program.methods
//       .addSolFeedData(period, days)
//       .accounts({
//         historyBuffer: historyBuffer,
//         feedAcc: feedAccPubkey,
//         authority: authority.publicKey,
//       })
//       .signers([authority.payer])
//       .rpc();
//   });
// });

// describe("calculating data spread!", async () => {
//   it("data spread...", async () => {
//     const sig = await program.methods
//       .dataSpreadCalculate()
//       .accounts({
//         feedVectorAcc: feedAccPubkey,
//         authority: authority.publicKey,
//       })
//       .signers([authority.payer])
//       .rpc();
//   });
// });

describe("fetch account", async () => {
  it("fetching!...", async () => {
    const feed = await program.account.solanaFeed.fetch(feedAccPubkey);
    console.log("days: " + feed.days.toNumber());

    console.log(feed.feed);
    console.log("Data Spread: " + feed.dataSpread);
  });
});

// describe("wiping out the feed account!", async () => {
//   it("wiping...", async () => {
//     const sig = await program.methods
//       .emptyAccount()
//       .accounts({
//         feedVecAcc: feedAccPubkey,
//         authority: authority.publicKey,
//       })
//       .signers([authority.payer])
//       .rpc();
//   });
// });
