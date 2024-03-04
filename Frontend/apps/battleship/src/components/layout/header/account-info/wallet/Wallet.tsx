import { AccountButton } from '../account-button';
import styles from './Wallet.module.scss';

type Props = {
  balance: {
    value: string;
    unit: string;
  } | undefined;//Account['balance'];
  address: string;
  name: string | undefined;
  onClick: () => void;
};

function Wallet({ balance, address, name, onClick }: Props) {
  return (
    <div className={styles.wallet}>
      <div className={styles.balanceContainer}>
        <p className={styles.balanceText}>Balance:</p>
        <p className={styles.balance}>
          {balance?.value}<span className={styles.currency}>{" " + balance?.unit}</span>
        </p>
      </div>
      <AccountButton address={address} name={name} onClick={onClick} />
    </div>
  );
}

export { Wallet };
