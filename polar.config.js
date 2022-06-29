const accounts = [
  {
    name: 'account_0',
    address: 'secret1nmjq4cqdwr9d68nun3kkxr9h2z5l5u03ypmyk2',
    mnemonic: 'hood endorse primary mixed camera piece agree chimney upgrade album blade alcohol reunion forget invite squeeze general bench aerobic happy saddle off buddy soccer'
  },
  {
    name: 'account_1',
    address: 'secret1ddfphwwzqtkp8uhcsc53xdu24y9gks2kug45zv',
    mnemonic: 'sorry object nation also century glove small tired parrot avocado pulp purchase'
  }
];

module.exports = {
  networks: {
    default: {
      endpoint: 'http://localhost:1337/',
      accounts: accounts,
    },
    development: {
      endpoint: 'tcp://0.0.0.0:26656',
      nodeId: '115aa0a629f5d70dd1d464bc7e42799e00f4edae',
      chainId: 'enigma-pub-testnet-3',
      keyringBackend: 'test',
      types: {}
    },
    // Supernova Testnet
    testnet: {
      endpoint: 'http://40.88.137.151:1317',
      chainId: 'pulsar-2',
      trustNode: true,
      keyringBackend: 'test',
      accounts: accounts,
      types: {},
      fees: {
        upload: {
            amount: [{ amount: "500000", denom: "uscrt" }],
            gas: "2000000",
        },
        init: {
            amount: [{ amount: "125000", denom: "uscrt" }],
            gas: "500000",
        },
      }
    }
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.55.0",
  }
};