import { XOR_NN_CONTRACT } from '@/app/consts';
import { useEffect, useState } from 'react';
import { Subscriptions, CardNNData } from '@/components';
import { useContractUtils } from '@/app/hooks';
import { TemplateAlertOptions, useAccount, useAlert } from '@gear-js/react-hooks';
import { XorNumSelection, ButtonPredict } from '@/components';
import './XorDemoNN.scss';

export default function XorDemoNN() {
  const { account } = useAccount();
  const alert = useAlert();
  const { readState } = useContractUtils();
  const [userIsSubscribed, setUserIsSubscribed] = useState(false);
  const [firstValue, setFirstValue] = useState(0);
  const [secondValue, setSecondValue] = useState(0);
  const [prediction, setPrediction] = useState(0);

  const userDoASubscription = () => {
    setUserIsSubscribed(true);
  };

  const handlePredictionMessage = async () => {
    if (!account) {
        alert.error("Account is not started");
        return;
    }

    const {userLastPrediction}: any = await readState(
        XOR_NN_CONTRACT.programId,
        XOR_NN_CONTRACT.programMetadata,
        {
            UserLastPrediction: account.decodedAddress
        }
    );

    const [ num, den ] = userLastPrediction.split(' / ');

    const probability = num / den;
    
    if (probability >= 0.9) {
        setPrediction(1);
    } else {
        setPrediction(0);
    }

    const alertOptions: TemplateAlertOptions = {
        title: "NeuroShark",
      };

    alert.info(`probability of it being 1: ${(num / den) * 100}%`, alertOptions);
  };

  const dataToSendToNN = () => {
    if (firstValue === 0 && secondValue === 0) {
      return { Predict: [{ Zero: null }, { Zero: null }] };
    } else if (firstValue === 1 && secondValue === 0) {
      return { Predict: [{ One: null }, { Zero: null }] };
    } else if (firstValue === 0 && secondValue === 1) {
      return { Predict: [{ Zero: null }, { One: null }] };
    }

    return { Predict: [{ One: null }, { One: null }] };
  };

  const isSubscribed = async () => {
    if (!account) {
      alert.error('Account is not initialized');
      return;
    }

    const { userIsSubscribed }: any = await readState(XOR_NN_CONTRACT.programId, XOR_NN_CONTRACT.programMetadata, {
      UserIsSubscribed: account.decodedAddress,
    });

    setUserIsSubscribed(userIsSubscribed);

    if (!userIsSubscribed) {
      return;
    }
  };

  useEffect(() => {
    async function test() {
      await isSubscribed();
    }

    test();
  }, [account, userIsSubscribed, userDoASubscription]);

  return (
    <div className="xor-demo-nn">
      {userIsSubscribed ? (
        <div className="xor-demo-nn__work-space">
          <h1 className="xor-demo-nn__tittle">Xor Gate</h1>
          <div className="xor-demo-nn__container">
            <XorNumSelection onNumChanged={setFirstValue} />
            <p className="xor-demo-nn__char">⊕</p>
            <XorNumSelection onNumChanged={setSecondValue} />
            <p className="xor-demo-nn__char">=</p>
            <XorNumSelection changeValueTo={prediction} />
          </div>
          <ButtonPredict
            onPredict={handlePredictionMessage}
            available={true}
            payload={dataToSendToNN()}
            programId={XOR_NN_CONTRACT.programId}
            programMetadata={XOR_NN_CONTRACT.programMetadata}
          />
        </div>
      ) : (
        <div className="account__subscriptions-container">
          <Subscriptions onSubscription={userDoASubscription} />
        </div>
      )}
    </div>
  );
}
