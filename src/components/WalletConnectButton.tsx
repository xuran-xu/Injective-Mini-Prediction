import React from 'react';

interface WalletConnectButtonProps {
  isConnected: boolean;
  onConnect: () => void;
  onDisconnect: () => void;
}

const WalletConnectButton: React.FC<WalletConnectButtonProps> = ({ isConnected, onConnect, onDisconnect }) => {
  return (
    <button onClick={isConnected ? onDisconnect : onConnect}>
      {isConnected ? 'Disconnect Wallet' : 'Connect Wallet'}
    </button>
  );
};

export default WalletConnectButton;
