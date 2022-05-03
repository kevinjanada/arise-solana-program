import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AriseSolanaProgram } from "../target/types/arise_solana_program";
import { expect } from 'chai';
import { PublicKey } from '@solana/web3.js';

describe("arise-solana-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AriseSolanaProgram as Program<AriseSolanaProgram>;

  it("Create user data and update email!", async () => {
    const wallet = (program.provider as anchor.AnchorProvider).wallet

    const arbitraryWallet = anchor.web3.Keypair.generate()

    const [userDataPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("user-data"),
          provider.wallet.publicKey.toBuffer(),
          // this would cause
          // Error: failed to send transaction: Transaction simulation failed:
          // Error processing Instruction 0:
          // Cross-program invocation with unauthorized signer or writable account
          //arbitraryWallet.publicKey.toBuffer(),
        ],
        program.programId
      );
    await program.methods
      .createUserData('kevin.janada@gmail.com')
      .accounts({
        user: wallet.publicKey,
        // This would cause Error: Signature verification failed
        // because the first signer in account is wallet. set in Anchor.toml 
        //user: arbitraryWallet.publicKey,
        userData: userDataPDA,
      })
      .rpc()

    const userDataState = await program.account.userData.fetch(userDataPDA)
    expect(userDataState.email).to.equal('kevin.janada@gmail.com')
    expect(userDataState.publicKey.toBase58()).to.equal(wallet.publicKey.toBase58())

    await program.methods
      .updateUserDataEmail('dennis.darwis@gmail.com')
      .accounts({
        user: wallet.publicKey,
        userData: userDataPDA
      })
      .rpc()

    expect((await program.account.userData.fetch(userDataPDA)).email)
      .to.equal('dennis.darwis@gmail.com')
      
  });
});
