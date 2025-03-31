import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';

import { RadiyumPricetwo } from '../target/types/radiyum_pricetwo';

describe("radiyum-price", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RadiyumPricetwo as Program<RadiyumPricetwo>;

  it("fetch!", async () => {
    // Add your test here.
    // const tokenA = "So11111111111111111111111111111111111111112"; // Token A of the pool; using Wrapped SOL (WSOL) as the default
    // const tokenB = "8ihkFrPHVb8SEv61LMye84HE78zCYqjNAwxb1SBQpump";

    // const secretKeyNum = [157,208,102,68,11,180,142,80,113,243,54,33,223,220,226,29,97,97,240,52,206,235,81,69,215,1,114,96,71,41,65,141,240,201,168,238,34,23,110,142,112,187,167,148,85,203,224,120,17,250,166,110,218,193,194,175,64,246,200,185,12,158,221,16];//[152,113,217,183,10,211,184,243,146,246,254,96,185,33,45,131,72,146,200,218,176,94,150,55,40,203,99,33,105,215,45,49,216,131,210,128,45,129,114,232,47,233,187,224,252,34,168,49,234,174,20,177,3,159,232,182,169,203,136,137,3,143,187,29];

    // const secretKey = Uint8Array.from(secretKeyNum);
    // const keypair = Keypair.fromSecretKey(secretKey);
    // const provider = anchor.AnchorProvider.env();

  try{
   
    const txHash = await program.methods
      .fetchPumpPrice()
      .accounts({
        quoteVault: new PublicKey(
          "4mfz6gWfLHh2WDmCo9jp4CQeyUnhrtGjpGkuaH8jzDRj"
        ),
        baseVault: new PublicKey(
          "3LQaWf23GUbs8di7Do5a2ayHKEBrWWtb9JYizbSZJysN"
        ),
        baseToken: new PublicKey(
          "8ihkFrPHVb8SEv61LMye84HE78zCYqjNAwxb1SBQpump"
        )

      })
      .signers([])
      .simulate();
    console.log(`Use 'solana confirm -v ${txHash.raw}' to see the logs`);
  }catch(e){
    console.error('Error in transaction:', e);
  }
  });
});
