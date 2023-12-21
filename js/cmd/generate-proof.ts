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
    "inj1tlqtznd9gh0a53krerduzdsfadafamag0svccu:100000000", // YoshiMitsu
    "inj1q08vl6nwcqe9hm29pg6ral02uh45rnakcuhajf:100000000", // user1
    "inj1gfy4t9a9fafhf740tvxc0qw257ypnu8ppdqflm:100000000", // user2
    "inj18f2tffku9cqnyup73sx8qyac4cnk6xxj886a8w:100000000", // user3
    "inj1wmpgeccf07zyew0zvr5qy3wynjervphvhjf945:100000000", // user4
  ].map((x) => SHA256(x));

  const tree = new MerkleTree(leaves, SHA256, {
    sort: true,
  });
  const root = tree.getRoot().toString("hex");
  const leaf = leaves[1];
  let proof = tree.getProof(leaf as any);

  console.log(`root: "${root}"`);
  for (const p of proof) {
    console.log(`"${p.data.toString("hex")}",`);
  }
};
