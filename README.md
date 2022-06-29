# Whisprs

Whisprs is a proof-of-concept for private messaging on Cosmos networks, powered by Secret Network.

It uses permits to allow users to send private messages to addresses either on Secret, or even on other Cosmos chains.

This is a very basic and bare-bones contract, with more advanced features such as event notification and delegated permissions 
possible future additions.

# Contract Handles
- `SendMemo`: Send a private message to another address 
- `SetViewingKey`: Set a viewing key for a user if he prefers it over a permit

# Contract Queries
- `GetMemo`: Get paginated list of messages for a specific user.

# Compiling contracts

Use this command to compile your contracts: 
`polar compile`

# Deploy Contracts

`polar run scripts/deploy.js`