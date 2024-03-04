import { SUBSCRIPTIONS_CONTRACT } from '@/app/consts';
import { useAlert, useAccount, useApi } from '@gear-js/react-hooks';
import { useVoucherUtils, useContractUtils } from '@/app/hooks';
import { useGearHooks } from '@/routes/root';
import './ButtonSubscribe.scss';
import { ProgramMetadata } from '@gear-js/api';

export type SubscriptionCost = 'free' | 'basic' | 'ultimate';

export interface ButtonSubscribeProps {
  cost: SubscriptionCost;
  onSubscribe: any;
  available: boolean;
}

export function ButtonSubscribe({ cost, onSubscribe, available }: ButtonSubscribeProps) {
  const { api } = useApi();
  const alert = useAlert();
  const { account } = useAccount();
  const { generateNewVoucher, accountVoucherId, voucherExists, checkVoucherForUpdates } = useVoucherUtils();
  const { sendMessageWithVoucher, signMessageTransaction } = useContractUtils();

  const signSubscriptionTransaction = async () => {
    if (!account) {
      console.log('Account is not initialized');
      return;
    }

    const voucherId = await accountVoucherId(SUBSCRIPTIONS_CONTRACT.programId, account.decodedAddress);

    await checkVoucherForUpdates(voucherId as `0x${string}`, account.decodedAddress);

    let price = 0;
    let payload = null;

    if (cost == 'basic') {
      price = 3;
      payload = { Subscribe: { Basic: null } };
    } else if (cost == 'ultimate') {
      price = 5;
      payload = { Subscribe: { Ultimate: null } };
    } else {
      payload = { Subscribe: { Free: null } };
    }

    try {
      await sendMessageWithVoucher(
        account.decodedAddress,
        account.meta.source,
        SUBSCRIPTIONS_CONTRACT.programId,
        ProgramMetadata.from(SUBSCRIPTIONS_CONTRACT.programMetadata),
        payload,
        price,
        'Finalized',
        'Error when subscribing',
        'Subscribing',
        'NeuroShark: ',
      );

      console.log("SE TERMINO EL PDO DE SUBSCRIBIRSE, AHORA SE MANDARA QUE YA SE FIRMO!");
      
      if (onSubscribe) {
        console.log("SI SE MANDO UNA FUNCION PARA SUBSCRIBIRSE!");
        
        onSubscribe(false);
      }
    } catch (e) {
      console.log('Error while subscribing');
    }
  };

  const handleSubscript = async () => {
    if (!api || !account) {
      alert.error('Api does not initialized!!!');
      return;
    }

    if (await voucherExists(SUBSCRIPTIONS_CONTRACT.programId, account.decodedAddress)) {
      console.log('Voucher exists');
      await signSubscriptionTransaction();
      return;
    }

    await generateNewVoucher(SUBSCRIPTIONS_CONTRACT.programId);

    await signSubscriptionTransaction();
  };

  return (
    <button
      className={`button-subscribe${!available ? ' button-subscribe--disabled' : ' button-subscribe--hover'}`}
      onClick={handleSubscript}
      disabled={!available}>
      {available ? 'Subscribe' : 'Soon'}
    </button>
  );
}
