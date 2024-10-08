import React, { useState, useEffect } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, Provider, web3 } from '@project-serum/anchor';
import idl from './idl.json';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

const programID = new PublicKey(idl.metadata.address);
const opts = {
  preflightCommitment: "processed"
}

function App() {
  const [walletAddress, setWalletAddress] = useState(null);
  const [provider, setProvider] = useState(null);

  useEffect(() => {
    const onLoad = async () => {
      await checkIfWalletIsConnected();
    };
    window.addEventListener('load', onLoad);
    return () => window.removeEventListener('load', onLoad);
  }, []);

  const checkIfWalletIsConnected = async () => {
    try {
      const { solana } = window;

      if (solana) {
        if (solana.isPhantom) {
          console.log('Phantom wallet found!');
          const response = await solana.connect({ onlyIfTrusted: true });
          console.log(
            'Connected with Public Key:',
            response.publicKey.toString()
          );
          setWalletAddress(response.publicKey.toString());
          setupProvider();
        }
      } else {
        alert('Solana object not found! Get a Phantom Wallet 👻');
      }
    } catch (error) {
      console.error(error);
    }
  };

  const connectWallet = async () => {
    const { solana } = window;
  
    if (solana) {
      const response = await solana.connect();
      console.log('Connected with Public Key:', response.publicKey.toString());
      setWalletAddress(response.publicKey.toString());
      setupProvider();
    }
  };

  const setupProvider = () => {
    const wallet = new PhantomWalletAdapter();
    const connection = new Connection(web3.clusterApiUrl('devnet'), opts.preflightCommitment);
    const provider = new Provider(connection, wallet, opts);
    setProvider(provider);
  }

  const createStream = async (amount, duration, recipient) => {
    try {
      const program = new Program(idl, programID, provider);
      await program.rpc.createStream(
        new BN(amount),
        new BN(duration),
        {
          accounts: {
            sender: provider.wallet.publicKey,
            recipient: new PublicKey(recipient),
            stream: getProgramDerivedAddress(provider.wallet.publicKey, new PublicKey(recipient)),
            senderToken: await getAssociatedTokenAddress(provider.wallet.publicKey),
            vault: await getAssociatedTokenAddress(getProgramDerivedAddress(provider.wallet.publicKey, new PublicKey(recipient))),
            systemProgram: web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: web3.SYSVAR_RENT_PUBKEY,
          },
        }
      );
      console.log("Stream created successfully");
    } catch (error) {
      console.log("Error creating stream:", error);
    }
  }

  const withdraw = async () => {
    try {
      const program = new Program(idl, programID, provider);
      await program.rpc.withdraw({
        accounts: {
          sender: new PublicKey("SENDER_PUBLIC_KEY"),
          recipient: provider.wallet.publicKey,
          stream: getProgramDerivedAddress(new PublicKey("SENDER_PUBLIC_KEY"), provider.wallet.publicKey),
          vault: await getAssociatedTokenAddress(getProgramDerivedAddress(new PublicKey("SENDER_PUBLIC_KEY"), provider.wallet.publicKey)),
          recipientToken: await getAssociatedTokenAddress(provider.wallet.publicKey),
          tokenProgram: TOKEN_PROGRAM_ID,
        },
      });
      console.log("Withdrawal successful");
    } catch (error) {
      console.log("Error withdrawing:", error);
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <h1>SolPay</h1>
        {!walletAddress && (
          <button onClick={connectWallet}>Connect Wallet</button>
        )}
        {walletAddress && (
          <div>
            <p>Connected: {walletAddress}</p>
            <button onClick={() => createStream(100, 86400, 'RECIPIENT_ADDRESS')}>Create Stream</button>
            <button onClick={withdraw}>Withdraw</button>
          </div>
        )}
      </header>
    </div>
  );
}

export default App;