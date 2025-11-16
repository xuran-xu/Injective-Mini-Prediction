import React from 'react';

interface Market {
  eventName: string;
  deadline: number;
  id: string; // Unique identifier for the market
}

interface MarketListProps {
  markets: Market[];
  onMarketSelect: (marketId: string) => void;
}

const MarketList: React.FC<MarketListProps> = ({ markets, onMarketSelect }) => {
  return (
    <div>
      <h2>Available Markets</h2>
      <ul>
        {markets.map((market) => (
          <li key={market.id} onClick={() => onMarketSelect(market.id)}>
            {market.eventName} - Deadline: {new Date(market.deadline).toLocaleString()}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default MarketList;
