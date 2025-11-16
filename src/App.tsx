import React, { useState } from 'react';
import MarketList from './components/MarketList';
import MarketDetail from './components/MarketDetail';
import WalletConnectButton from './components/WalletConnectButton';

interface Market {
  eventName: string;
  deadline: number;
  description: string;
  poolYes: number;
  poolNo: number;
  id: string;
  winningOption: string | null;
}

function App() {
  const [selectedMarketId, setSelectedMarketId] = useState<string | null>(null);
  const [markets, setMarkets] = useState<Market[]>([
    {
      eventName: 'Will it rain tomorrow?',
      deadline: Date.now() + 86400000, // 24 hours from now
      description: 'Predict whether it will rain tomorrow in London.',
      poolYes: 100,
      poolNo: 200,
      id: '1',
      winningOption: null,
    },
    {
      eventName: 'Who will win the election?',
      deadline: Date.now() + 172800000, // 48 hours from now
      description: 'Predict the winner of the upcoming election.',
      poolYes: 150,
      poolNo: 100,
      id: '2',
      winningOption: null,
    },
  ]);

  const [isConnected, setIsConnected] = useState(false);

  const handleMarketSelect = (marketId: string) => {
    setSelectedMarketId(marketId);
  };

  const handleStakeYes = (amount: number) => {
    console.log('Stake Yes:', amount);
    // Implement staking logic here
  };

  const handleStakeNo = (amount: number) => {
    console.log('Stake No:', amount);
    // Implement staking logic here
  };

  const handleClaimReward = () => {
    console.log('Claim Reward');
    // Implement claim reward logic here
  };

  const handleConnectWallet = () => {
    setIsConnected(true);
    // Implement wallet connection logic here
  };

  const handleDisconnectWallet = () => {
    setIsConnected(false);
    // Implement wallet disconnection logic here
  };

  const selectedMarket = selectedMarketId ? markets.find((market) => market.id === selectedMarketId) : null;

  return (
    <div>
      <h1>Injective Mini Prediction Market</h1>
      <WalletConnectButton
        isConnected={isConnected}
        onConnect={handleConnectWallet}
        onDisconnect={handleDisconnectWallet}
      />

      {selectedMarket ? (
        <MarketDetail
          eventName={selectedMarket.eventName}
          description={selectedMarket.description}
          poolYes={selectedMarket.poolYes}
          poolNo={selectedMarket.poolNo}
          deadline={selectedMarket.deadline}
          winningOption={selectedMarket.winningOption}
          onStakeYes={handleStakeYes}
          onStakeNo={handleStakeNo}
          canClaimReward={selectedMarket.winningOption !== null}
          onClaimReward={handleClaimReward}
        />
      ) : (
        <MarketList markets={markets} onMarketSelect={handleMarketSelect} />
      )}
    </div>
  );
}

export default App;
