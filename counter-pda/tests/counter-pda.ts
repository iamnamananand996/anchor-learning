import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CounterPda } from '../target/types/counter_pda';
import { PublicKey } from '@solana/web3.js';

describe('counter-pda', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterPda as Program<CounterPda>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const [counter_pda] = PublicKey.findProgramAddressSync(
    [Buffer.from('counter')],
    program.programId
  );

  it('Is initialized!', async () => {
    try {
      // const txSig = await program.methods.initialize().rpc();
      const transactionSignature = await program.methods.increment().rpc();

      const accountData = await program.account.counter.fetch(counter_pda);
      console.log(`Transaction Signature: ${transactionSignature}`);
      console.log(`Count: ${accountData.count}`);
    } catch (error) {
      // If PDA Account already created, then we expect an error
      console.log('', error);
    }
  });
});
