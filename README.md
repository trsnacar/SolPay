# SolPay

SolPay is an innovative payment solution for real-time micropayments and continuous revenue streams, running on the Solana blockchain.

## Features

- Instant Micropayments
- Payments in Flow
- Smart Contract Integration
- Multi-Currency Support

## Installation

1. Clone the repo:
   ```
   git clone https://github.com/yourusername/solpay.git
   ```

2. Install the dependencies:
   ```
   cd solpay
   npm install
   ```

3. Install Solana CLI and connect to devnet:
   ```
   solana config set --url https://api.devnet.solana.com
   ```

4. Create a new keypair:
   ```
   solana-keygen new -o id.json
   ```

5. Take Devnet SOL:
   ```
   solana airdrop 2 $(solana-keygen pubkey id.json)
   ```

## Usage

1. Build and deploy the smart contract:
   ```
   anchor build
   anchor deploy
   ```

2. Start the frontend application:
   ```
   cd app
   npm start
   ```

3. Go to `http://localhost:3000` in your browser and start using the app.

## Contributing

We welcome your contributions! Please open a topic to discuss your changes before submitting a pull request.

## License

This project is licensed under the MIT license. See the `LICENSE` file for more information.

Translated with DeepL.com (free version)