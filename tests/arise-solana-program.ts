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

    /**
      * Create new user data
      * */
    const email = 'kevin.janada@gmail.com'
    const seedPrefix = 'user-data'

    const [userDataPDA, _b] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(seedPrefix),
          anchor.utils.bytes.utf8.encode(email)
        ],
        program.programId
      );

    await program.methods
      .createUserData(email)
      .accounts({
        authority: wallet.publicKey,
        userData: userDataPDA,
      })
      .rpc()

    let userDataState = await program.account.userData.fetch(userDataPDA)
    expect(userDataState.email).to.equal('kevin.janada@gmail.com')
    expect(userDataState.authority.toBase58()).to.equal(wallet.publicKey.toBase58())

    /**
      * Update user data:
      * - it deletes the old user data
      * - creates a new user data
      * */
    const newEmail = 'kevinjrardian@gmail.com'
    const [newUserDataPDA, _c] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode(seedPrefix),
          anchor.utils.bytes.utf8.encode(newEmail)
        ],
        program.programId
      )
    await program.methods
      .updateUserData(newEmail)
      .accounts({
        authority: wallet.publicKey,
        existingUserData: userDataPDA,
        userData: newUserDataPDA,
      })
      .rpc()

    userDataState = await program.account.userData.fetch(newUserDataPDA)
    expect(userDataState.email).to.equal(newEmail)
    expect(userDataState.authority.toBase58()).to.equal(wallet.publicKey.toBase58())
  });
});
