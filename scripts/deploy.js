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

  const contract_info = await contract.instantiate({}, "ajsdfjasd66jfasjdf22fff-test", contract_owner);
  console.log(contract_info);

  const set_vk = await contract.tx.set_viewing_key(contract_owner, {key: "hello"});
  console.log(set_vk);

  const inc_response = await contract.tx.send_memo(contract_owner, {to: "secret1399pyvvk3hvwgxwt3udkslsc5jl3rqv4yshfrl", message: "hello son"});
  console.log(inc_response);

  // await contract.query.get_memo({
  //   address:"secret1399pyvvk3hvwgxwt3udkslsc5jl3rqv4yshfrl",
  //   auth: {permit:
  //         {params:{
  //           chain_id:"pulsar-2",
  //             permit_name:"memo_secret1rf03820fp8gngzg2w02vd30ns78qkc8rg8dxaq",
  //             allowed_tokens:["secret1rf03820fp8gngzg2w02vd30ns78qkc8rg8dxaq"],
  //             permissions:["history"]},
  //           signature:{"pub_key":{"type":"tendermint/PubKeySecp256k1","value":"A5M49l32ZrV+SDsPnoRv8fH7ivNC4gEX9prvd4RwvRaL"},"signature":"hw/Mo3ZZYu1pEiDdymElFkuCuJzg9soDHw+4DxK7cL9rafiyykh7VynS+guotRAKXhfYMwCiyWmiznc6R+UlsQ=="}}}})

}

module.exports = { default: run };
