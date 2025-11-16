import React, { useState } from 'react';

interface MarketDetailProps {
  eventName: string;
  description: string;
  poolYes: number;
  poolNo: number;
  deadline: number;
  winningOption: string | null;
  onStakeYes: (amount: number) => void;
  onStakeNo: (amount: number) => void;
  canClaimReward: boolean;
  onClaimReward: () => void;
}

const MarketDetail: React.FC<MarketDetailProps> = ({
  eventName,
  description,
  poolYes,
  poolNo,
  deadline,
  winningOption,
  onStakeYes,
  onStakeNo,
  canClaimReward,
  onClaimReward,
}) => {
  const [stakeAmount, setStakeAmount] = useState<number>(0);

  return (
    <div>
      <h2>{eventName}</h2>
      <p>{description}</p>
      <p>Pool (Yes): {poolYes}</p>
      <p>Pool (No): {poolNo}</p>
      <p>Deadline: {new Date(deadline).toLocaleString()}</p>

      {winningOption &amp;&amp; (
        <p>Winning Option: {winningOption}</p>
      )}

      <div>
        <input
          type="number"
          value={stakeAmount}
          onChange={(e) => setStakeAmount(parseFloat(e.target.value))}
        />
        <button onClick={() => onStakeYes(stakeAmount)}>Stake Yes</button>
        <button onClick={() => onStakeNo(stakeAmount)}>Stake No</button>
      </div>

      {canClaimReward &amp;&amp; (
        <button onClick={onClaimReward}>Claim Reward</button>
      )}
    </div>
  );
};

export default MarketDetail;
