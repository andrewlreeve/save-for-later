import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SaveForLater } from '../target/types/save_for_later';

describe('save-for-later', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SaveForLater as Program<SaveForLater>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
