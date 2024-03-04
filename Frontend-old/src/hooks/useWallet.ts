import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { LOCAL_STORAGE } from '../consts';
import { WALLET } from '../components/consts';
import { WalletId } from '../components/walletId';

export const useWallet = () => {
  const { accounts } = useAccount();
  const [walletId, setWalletId] = useState<WalletId | undefined>(localStorage[LOCAL_STORAGE.WALLET]);
  const resetWalletId = () => setWalletId(undefined);
  const getWalletAccounts = (id: WalletId) => accounts && accounts.filter(({ meta }) => meta.source === id);
  const saveWallet = () => walletId && localStorage.setItem(LOCAL_STORAGE.WALLET, walletId.toString());
  const removeWallet = () => localStorage.removeItem(LOCAL_STORAGE.WALLET);
  const wallet = walletId && WALLET;
  const walletAccounts = walletId && getWalletAccounts(walletId);
  return { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts, saveWallet, removeWallet };
}
