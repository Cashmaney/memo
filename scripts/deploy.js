const { Contract, getAccountByName } = require("secret-polar");

async function run () {
  const contract_owner = getAccountByName("account_0");
  const contract = new Contract("memo");
  await contract.parseSchema();

  const deploy_response = await contract.deploy(
    contract_owner,
    { // custom fees
      amount: [{ amount: "750000", denom: "uscrt" }],
      gas: "3000000",
    }
  );
  console.log(deploy_response);

  const contract_info = await contract.instantiate({}, "ajsdfjasd66jfasjdffff-test", contract_owner);
  console.log(contract_info);

  const set_vk = await contract.tx.set_viewing_key(contract_owner, {key: "hello"});
  console.log(set_vk);

  const inc_response = await contract.tx.send_memo(contract_owner, {to: "secret1399pyvvk3hvwgxwt3udkslsc5jl3rqv4yshfrl", message: "hello son"});
  console.log(inc_response);
}

module.exports = { default: run };
