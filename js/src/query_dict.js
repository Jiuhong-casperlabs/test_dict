import { CasperClient, Contracts } from "casper-js-sdk";

const main = async () => {
  const client = new CasperClient("http://3.140.179.157:7777/rpc");

  const { Contract } = Contracts;

  const contractClient = new Contract(client);
  const contract_hash =
    "hash-bbdbf32fcc89b9113fe951f82c55c8af88e1ab1b058ac55d4ba785b434ff9734"; //
  contractClient.setContractHash(contract_hash);

  console.log("queryContractDictionary");
  const result = await contractClient.queryContractDictionary(
    "count_uref",
    "count"
  );

  console.log("result", result.data.toString());
};

main();
