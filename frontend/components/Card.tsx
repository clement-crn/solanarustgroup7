import { useEffect, useState } from 'react';
import { Web3ReactSelectedHooks } from '@web3-react/core';
import { Connector } from '@web3-react/types';
import { PublicKey, SystemProgram, Connection, Transaction, clusterApiUrl } from '@solana/web3.js';
import { AnchorProvider, Program } from '@project-serum/anchor';
import idl from './idl.json'; // Replace with the actual IDL file for your program

const PROGRAM_ID = new PublicKey('892sb2f1GsHR8i5zXYgtKdVTMhSPWLRGLKiVD2Sz1KKp');

export default function Card({
  connector,
  hooks,
  name,
}: {
  connector: Connector;
  hooks: Web3ReactSelectedHooks;
  name: string;
}) {
  const {
    useSelectedAccount,
    useSelectedChainId,
    useSelectedIsActive,
    useSelectedIsActivating,
  } = hooks;

  const isActivating = useSelectedIsActivating(connector);
  const isActive = useSelectedIsActive(connector);
  const account = useSelectedAccount(connector);
  const chain = useSelectedChainId(connector);

  const [error, setError] = useState<Error | undefined>(undefined);
  const [connectionStatus, setConnectionStatus] = useState('Disconnected');
  const [campaignName, setCampaignName] = useState('');
  const [description, setDescription] = useState('');
  const [targetAmount, setTargetAmount] = useState('');
  const [provider, setProvider] = useState<any>(null); // Adding provider state
  const [userAccount, setUserAccount] = useState<string | null>(null); // State to store the account
  const [userChain, setUserChain] = useState<string | null>(null); // State to store the chain ID

  const handleToggleConnect = async () => {
    setError(undefined); // Clear error state

    if (isActive) {
      if (connector?.deactivate) {
        void connector.deactivate();
      } else {
        void connector.resetState();
      }
    } else if (!isActivating) {
      setConnectionStatus('Connecting..');
      try {
        const provider = await window.phantom.solana.connect();
        setProvider(provider); // Set provider state once connected
        setConnectionStatus('Connected');
        console.log('Connected to Phantom:', provider.publicKey.toString());

        // Set the account and chain after successful connection
        setUserAccount(provider.publicKey.toString());
        setUserChain('devnet'); // or get the chain programmatically if needed
      } catch (e) {
        setError(e);
        setConnectionStatus('Connection Failed');
      }
    }
  };

  useEffect(() => {
    // Automatically connect if trusted
    if (window.phantom && window.phantom.solana) {
      window.phantom.solana.connect({ onlyIfTrusted: true })
        .then((resp: any) => {
          setProvider(resp);
          setConnectionStatus('Connected');
          console.log('Eagerly connected to Phantom:', resp.publicKey.toString());

          // Set the account and chain after successful connection
          setUserAccount(resp.publicKey.toString());
          setUserChain('devnet'); // or get the chain programmatically if needed
        })
        .catch(() => {
          setConnectionStatus('Disconnected');
        });
    }
  }, []);

  const createCampaign = async () => {
    try {
      if (!userAccount) {
        throw new Error('Wallet not connected or invalid');
      }

      // Validate account as PublicKey
      const walletPublicKey = new PublicKey(userAccount);

      const connection = new Connection(clusterApiUrl('devnet'), 'processed');
      const wallet = {
        publicKey: walletPublicKey,
        signTransaction: async (tx: Transaction) => {
          const signedTx = await connector.activate(tx);
          return signedTx;
        },
      };
      const provider = new AnchorProvider(connection, wallet, {
        preflightCommitment: 'processed',
      });
      const program = new Program(idl as any, PROGRAM_ID, provider);

      const [campaignPda] = await PublicKey.findProgramAddress(
        [Buffer.from('campaign'), walletPublicKey.toBuffer()],
        PROGRAM_ID
      );

      const tx = await program.methods
        .createCampaign(campaignName, description, new web3.BN(parseFloat(targetAmount)))
        .accounts({
          campaign: campaignPda,
          creator: walletPublicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log(`Campaign created! Transaction ID: ${tx}`);
    } catch (err) {
      setError(err as Error);
      console.error('Error creating campaign:', err);
    }
  };

  return (
    <div>
      <p>{name.toUpperCase()}</p>
      <h3>Status - {error?.message ? `Error: ${error.message}` : connectionStatus}</h3>
      <h3>Address - {userAccount ? userAccount : 'No Account Detected'}</h3>
      <h3>ChainId - {userChain ? userChain : 'No Chain Connected'}</h3>
      <button onClick={handleToggleConnect} disabled={false}>
        {isActive ? 'Disconnect' : 'Connect'}
      </button>

      {isActive && (
        <div style={{ marginTop: '20px' }}>
          <h2>Create a Campaign</h2>
          <input
            type="text"
            placeholder="Campaign Name"
            value={campaignName}
            onChange={(e) => setCampaignName(e.target.value)}
          />
          <input
            type="text"
            placeholder="Description"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
          <input
            type="number"
            placeholder="Target Amount (SOL)"
            value={targetAmount}
            onChange={(e) => setTargetAmount(e.target.value)}
          />
          <button onClick={createCampaign}>Create Campaign</button>
        </div>
      )}
    </div>
  );
}
