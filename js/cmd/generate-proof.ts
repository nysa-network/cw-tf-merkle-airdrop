import { MerkleTree } from "merkletreejs";
import SHA256 from "crypto-js/sha256";

export const command = "generate-proof";

export const describe = "Generate a merkle proof";

export const builder = {
  // file: {
  //   type: String,
  //   require: true,
  // },
  // addr: {
  //   type: String,
  //   require: true,
  // },
};

export const handler = async function (argv: any) {
  let leaves = [
    // Inj addresses
    "inj1xxx:10",
    "inj1yyy:10",
    // Chihuahua addresses
    "chihuahua1xxx:10",
    "chihuahua1yyy:10",
  ].map((x) => SHA256(x));

  // let leaves = [
  //   "8bd1a3b12cb2fd35eda1fd59edb390045a85e88304d4d10a4494907627cff5e5",
  //   "ff245e84595d53a3356fddcf73f177b130670f26b82206b1040f0c4de07aa8ea",
  // ];

  const tree = new MerkleTree(leaves, SHA256, {});
  const root = tree.getRoot().toString("hex");
  const leaf = SHA256("inj1yyy:10");
  let proof = tree.getProof(leaf as any);

  // proof.map((x: any) => {lol: x, str: x.data.toString()});
  // proof.map((x: any) => { ...x});
  console.log("root:", root);
  for (const p of proof) {
    console.log(p.data.toString("hex"));
  }

  // console.log(proof.toString());

  // const { net, amount, mnemonic } = argv;
};
