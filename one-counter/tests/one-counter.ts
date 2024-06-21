import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { OneCounter } from '../target/types/one_counter';
import {
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';

describe('one-counter', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.OneCounter as Program<OneCounter>;

  const counterAccount = new Keypair();

  // it('Is initialized!', async () => {
  //   // Add your test here.
  //   const tx = await program.methods
  //     .initialize()
  //     .accounts({ user: wallet.publicKey, counter: counterAccount.publicKey })
  //     .instruction();
  //   console.log('Your transaction signature', tx);

  //   const transaction = new Transaction().add(tx);
  //   await sendAndConfirmTransaction(connection, transaction, [
  //     wallet.payer,
  //     counterAccount,
  //   ]);
  // });

  it('Is initialized', async () => {
    // Invoke the initialize instruction
    const transactionSignature = await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .signers([counterAccount]) // include counter keypair as additional signer
      .rpc({ skipPreflight: true });

    // Fetch the counter account data
    const accountData = await program.account.counter.fetch(
      counterAccount.publicKey
    );

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });

  it('Increment', async () => {
    // Invoke the increment instruction
    const transactionSignature = await program.methods
      .increment()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    // Fetch the counter account data
    const accountData = await program.account.counter.fetch(
      counterAccount.publicKey
    );

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
});
