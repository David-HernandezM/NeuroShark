import { SUBSCRIPTIONS_CONTRACT } from '@/app/consts';
import { useAlert, useAccount, useApi } from '@gear-js/react-hooks';
import { useVoucherUtils, useContractUtils } from '@/app/hooks';
import { useGearHooks } from '@/routes/root';
import './ButtonPrediction.scss';
import { ProgramMetadata } from '@gear-js/api';


interface ButtonPredictXorProps {
  onPredict: any;
  available: boolean;
  payload: any,
  programId: `0x${string}`,
  programMetadata: string
}

export function ButtonPredict({ onPredict, available, payload, programId, programMetadata }: ButtonPredictXorProps) {
  const { account } = useAccount();
  const { sendMessage } = useContractUtils();

  const signPredictionTransaction = async () => {
    if (!account) {
      console.log('Account is not initialized');
      return;
    }

    console.log("LO QUE SE MANDARA COMO PAYLOAD:");
    console.log(payload);  

    try {
      await sendMessage(
        account.decodedAddress,
        account.meta.source,
        programId,
        ProgramMetadata.from(programMetadata),
        payload,
        0,
        'Finalized',
        'Error when sending data',
        'Predicting...',
        'NeuroShark: ',
      );

      console.log("SE TERMINO EL PDO DE SUBSCRIBIRSE, AHORA SE MANDARA QUE YA SE FIRMO!");
      
      if (onPredict) {
        console.log("SI SE MANDO UNA FUNCION PARA SUBSCRIBIRSE!");
        onPredict();
      }
    } catch (e) {
      console.log('Error while subscribing');
    }
  };

  return (
    <button
      className={`button-predict${!available ? ' button-predict--disabled' : ' button-predict--hover'}`}
      onClick={signPredictionTransaction}
      disabled={!available}>
      {available ? 'Predict' : 'Soon'}
    </button>
  );
}