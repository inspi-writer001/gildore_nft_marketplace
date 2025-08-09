import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorMarketplace } from "../target/types/anchor_marketplace";
import admin_wallet_file from "./wallets/admin-wallet.json";
import user1_wallet_file from "./wallets/user-1-wallet.json";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

import path from "path";

let admin_wallet = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(admin_wallet_file)
);

let user_1 = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(user1_wallet_file)
);

describe("anchor_marketplace", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .anchorMarketplace as Program<AnchorMarketplace>;

  // setup for Umi

  const umi = createUmi(anchor.getProvider().connection);
  let keypair = umi.eddsa.createKeypairFromSecretKey(
    new Uint8Array(admin_wallet_file)
  );
  const signer = createSignerFromKeypair(umi, keypair);

  umi.use(irysUploader());
  umi.use(signerIdentity(signer));

  let asset: anchor.web3.Keypair;

  let marketplace = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("marketplace"), admin_wallet.publicKey.toBuffer()],
    program.programId
  )[0];
  let treasury = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), marketplace.toBuffer()],
    program.programId
  )[0];

  it.skip("Is initialized!", async () => {
    try {
      let name_of_program = "Gildore Marketplace";

      console.log(treasury);
      const tx = await program.methods
        .initialize({
          feeBps: 100,
          name: name_of_program,
        })
        .accounts({
          admin: admin_wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          // @ts-ignore
          // treasury
        })
        .signers([admin_wallet])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      if (error.logs) {
        console.log(error.logs);
      }
    }
  });

  it.skip("should fetch all marketplace accounts", async () => {
    try {
      let marketplace_accounts = await program.account.marketplace.all();
      console.log(marketplace_accounts);
    } catch (error) {
      console.log("failed to fetch  accounts from contract state");
      console.log(error);
    }
  });

  it("should create asset", async () => {
    try {
      asset = anchor.web3.Keypair.generate();
      console.log("asset: ", asset.publicKey.toBase58());
      console.log("admin: ", admin_wallet.publicKey.toBase58());
      const filePath = path.join(__dirname, "silver_image.jpg");
      const file = await readFile(filePath);
      //2. Convert image to generic file.
      const converted_file = createGenericFile(file, "silver_image.jpg", {
        contentType: "img/jpg",
      });
      //3. Upload image
      const [my_uri] = await umi.uploader.upload([converted_file]);
      console.log("my image uri: ", my_uri);

      const metadata = {
        name: "Silver Bar",
        symbol: "SLV",
        description: "Silver Bar with 999 purity",
        image: my_uri,
        attributes: [
          { trait_type: "purity", value: "999" },
          { trait_type: "weight", value: "125 KG" },
        ],
        properties: {
          files: [
            {
              type: "image/jpg",
              uri: my_uri,
            },
          ],
        },
        creators: [admin_wallet.publicKey.toBase58()],
      };
      const metadataUri = await umi.uploader.uploadJson(metadata);

      const tx = await program.methods
        .createNft({
          name: "Silver Bar",
          uri: metadataUri,
        })
        .accounts({
          asset: asset.publicKey,
          collection: null,
          creator: admin_wallet.publicKey,
        })
        // .remainingAccounts([
        //   {
        //     pubkey: asset.publicKey,
        //     isSigner: true,
        //     isWritable: true
        //   }
        // ])
        .signers([asset, admin_wallet])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      if (error.logs) {
        console.log(error.logs);
        throw Error("error occured");
      }
      throw Error(error);
    }
  });

  it("should list Asset for sale in Marketplace", async () => {
    try {
      // asset = {
      //   publicKey: new anchor.web3.PublicKey(
      //     "EcCEUsUb8ERyKHWCCih1hfRdZZ5uxCo8r2m3Erq42a2g"
      //   ),
      // };
      const tx = await program.methods
        .listNft({
          tokenId: 20051,
          price: new anchor.BN(500_000_000),
        })
        .accounts({
          asset: asset.publicKey,
          collection: null,
          seller: admin_wallet.publicKey,
        })
        .signers([admin_wallet])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      if (error.logs) {
        console.log(error.logs);
        throw Error("error occured");
      }
    }
  });

  it("should purchase Asset on sale in Marketplace", async () => {
    try {
      // asset = {
      //   publicKey: new anchor.web3.PublicKey(
      //     "EcCEUsUb8ERyKHWCCih1hfRdZZ5uxCo8r2m3Erq42a2g"
      //   ),
      // };
      const tx = await program.methods
        .purchaseNft()
        .accounts({
          asset: asset.publicKey,
          collection: null,
          seller: admin_wallet.publicKey,
          buyer: user_1.publicKey,
        })
        .signers([user_1])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      if (error.logs) {
        console.log(error.logs);
        throw Error("error occured");
      }
    }
  });

  it.skip("should burn purchased Asset for physical redemption", async () => {
    try {
      // asset = {
      //   publicKey: new anchor.web3.PublicKey(
      //     "EcCEUsUb8ERyKHWCCih1hfRdZZ5uxCo8r2m3Erq42a2g"
      //   ),
      // };
      const tx = await program.methods
        .redeemAsset()
        .accounts({
          asset: asset.publicKey,
          owner: user_1.publicKey,
          seller: admin_wallet.publicKey,
        })
        .signers([user_1])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      if (error.logs) {
        console.log(error.logs);
      }
      throw Error("error occured");
    }
  });
});
