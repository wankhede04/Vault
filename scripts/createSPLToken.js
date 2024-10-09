const anchor = require("@coral-xyz/anchor");
const { createMint, getMint, getOrCreateAssociatedTokenAccount, mintTo, getAccount } = require("@solana/spl-token");
const web3 = require("@solana/web3.js");
const {
  clusterApiUrl,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

const localRPC = `http://localhost:8899`;

// const connectionDevnet = new web3.Connection(web3.clusterApiUrl('devnet'), 'confirmed');
const connection = new web3.Connection(localRPC, "confirmed");

const fs = require("fs");

// Path to your Solana wallet file
const secretKeyPath = `${process.env.HOME}/.config/solana/id.json`;

// Read and parse the secret key from the file
const secretKey = Uint8Array.from(
  JSON.parse(fs.readFileSync(secretKeyPath, "utf8"))
);

airdropSOL = async (receiverPubKey, amountInSOL) => {
  const airdropSignature = await connection.requestAirdrop(
    receiverPubKey,
    LAMPORTS_PER_SOL * amountInSOL
  );
  const latestBlockHash = await connection.getLatestBlockhash();

  const balanceBefore = await connection.getBalance(receiverPubKey);
  console.log(
    `Balance before airdrop is : ${(
      balanceBefore / LAMPORTS_PER_SOL
    ).toString()}`
  );

  await connection.confirmTransaction({
    blockhash: latestBlockHash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: airdropSignature,
  });

  const balanceAfter = await connection.getBalance(receiverPubKey);
  console.log(
    `Balance after airdrop is : ${(
      balanceAfter / LAMPORTS_PER_SOL
    ).toString()}`
  );
};

createToken = async (mintAuthorityPubKey, freezeAuthorityPubKey, decimals, payerAccount) => {
    const mint = await createMint(
        connection,
        payerAccount,
        mintAuthorityPubKey,
        freezeAuthorityPubKey,
        decimals // same as SOL
    )

    console.log(`Token is deployed at account: ${mint.toBase58()}`);
    return mint;
}

async function getTokenInfo(connection, mint) {
  const tokenID = '2DU1onQ76pKf6XaRVt28vZnBXT8AiLh5XqsL6RfYGgKt';
  const mintInfo = await getMint(connection, mint);
  console.log("Mint Info: ", mintInfo.supply);
}



async function main() {
  const payer = Keypair.fromSecretKey(secretKey);
  const mintAuthorityPubKey = payer;
  const freezeAuthorityPubKey = payer;

  console.log("Public Key:", payer.publicKey.toString());

  await airdropSOL(payer.publicKey, 1);
  const TOKEN_DECIMALS = 9;

  const mint = await createToken(mintAuthorityPubKey.publicKey, freezeAuthorityPubKey.publicKey, TOKEN_DECIMALS, payer);

  await getTokenInfo(connection, mint);

  const tokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, mint, payer.publicKey);
  console.log("Associated token account: ", tokenAccount.address.toBase58());

  const tokenAccountInfo = await getAccount(
    connection,
    tokenAccount.address
  )
  console.log("Token balance before mint: ", tokenAccountInfo.amount);
  
  
  //Mint token
  await mintTo(
      connection,
      payer,
      mint,
      tokenAccount.address,
      mintAuthorityPubKey,
      100 * TOKEN_DECIMALS
    )

    const tokenAccountInfoAfter = await getAccount(
      connection,
      tokenAccount.address
    )
    console.log("Token balance after mint: ", ((tokenAccountInfoAfter.amount)/BigInt(TOKEN_DECIMALS)).toString());
}

main().catch(console.error);
