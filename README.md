On Stellar, fungible tokens implement the SEP-41 standard. A contract that creates other contracts (a factory) needs to deploy and initialize new token contracts .

A token contract typically has:

Admin: The address with minting/clawback privileges.

Metadata: Name, symbol, decimals (always 7 for Stellar assets).

Balances: Mapping of addresses to token balances.

Allowances: For delegated transfers .

Total Supply: The total number of tokens in existence.