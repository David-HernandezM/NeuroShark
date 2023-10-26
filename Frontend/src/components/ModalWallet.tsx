import { useWallet } from '@/hooks';
import { useAccount } from '@gear-js/react-hooks';
import { WALLETS } from './consts';

interface Props {
  onClose: () => void;
}

export const ModalWallet = () => {
  const { extensions, account, login, logout } = useAccount();

  const { getWalletAccounts, removeWallet, resetWalletId, saveWallet, setWalletId, wallet, walletAccounts } = useWallet();

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const isEnabled = extensions && extensions.some((extension) => extension.name === id);
      const status = isEnabled ? 'Enabled' : 'Disabled';

      const accountsCount = getWalletAccounts(id)?.length;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      const onClick = () => setWalletId(id);

      return (
        <li key={id}>
          <button type="button" onClick={onClick} disabled={!isEnabled}>
            <WalletItem icon={SVG} name={name} />

            <div>
              <p>{status}</p>
              {isEnabled && <p>{accountsStatus}</p>}
            </div>
          </button>
        </li>
      );
    });

  return (
    <div>ModalWallet</div>
  )
}
