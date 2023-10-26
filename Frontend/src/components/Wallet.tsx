import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { WalletIcon } from '@/Icons';

export const Wallet = () => {
  const { account, isAccountReady } = useAccount();

  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);
  return isAccountReady ? (
    <>
      {account ? (
        <button
          onClick={openModal}
          className='flex justify-center items-center space-x-2 rounded-full px-4 bg-gray-600'
        >
          <div className='w-5 h-5 rounded-full bg-red-500'></div>
          <p>{account.meta.name}</p>
        </button>
      ) : (
        <button
          onClick={openModal}
          type="button"
          className=" bg-black hover:bg-primary-100 ease-in duration-200 font-medium rounded-full px-5 py-2.5 text-center flex items-center space-x-2">
          <WalletIcon />
          <p>CONNECT</p>
        </button>
      )}
      {isModalOpen && <></>}
    </>
  ) : null
}
