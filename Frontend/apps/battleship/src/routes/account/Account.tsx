import { SUBSCRIPTIONS_CONTRACT } from '@/app/consts';
import { useEffect, useState } from 'react';
import { Subscriptions, CardNNData } from '@/components';
import { useContractUtils } from '@/app/hooks';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import XorImage from '@/assets/images/nn-cards-images/xor-gate.png';
import DoodleImage from '@/assets/images/nn-cards-images/doodle-image.png';
import './Account.scss';

export default function Account() {
  const { account } = useAccount();
  const alert = useAlert();
  const { readState } = useContractUtils();
  const [setsubscriptionName, setSetsubscriptionName] = useState('');
  const [userIsSubscribed, setUserIsSubscribed] = useState(false);

  const userDoASubscription = () => {
    setUserIsSubscribed(true);
  };

  const isSubscribed = async () => {
    if (!account) {
      alert.error('Account is not initialized');
      return;
    }

    const { userHasSubscription }: any = await readState(
      SUBSCRIPTIONS_CONTRACT.programId,
      SUBSCRIPTIONS_CONTRACT.programMetadata,
      {
        UserHasSubscription: account.decodedAddress,
      },
    );

    setUserIsSubscribed(userHasSubscription);

    if (!userHasSubscription) {
      return;
    }

    const { userSubscriptionType }: any = await readState(
      SUBSCRIPTIONS_CONTRACT.programId,
      SUBSCRIPTIONS_CONTRACT.programMetadata,
      {
        UserSubscriptionType: account.decodedAddress,
      },
    );

    setSetsubscriptionName(userSubscriptionType);

    if (userSubscriptionType === 'Free') {
      const nnData = await readState(SUBSCRIPTIONS_CONTRACT.programId, SUBSCRIPTIONS_CONTRACT.programMetadata, {
        UserSubscriptionType: account.decodedAddress,
      });
    }
  };

  useEffect(() => {
    async function test() {
      await isSubscribed();
    }

    test();
  }, [account, userIsSubscribed, userDoASubscription]);

  return (
    <div className="account">
      {userIsSubscribed ? (
        <>
          <h1 className="account__tittle">Neural Networks {` - ${setsubscriptionName}`}</h1>
          <div className="account__nn-cards-container">
            <CardNNData
              nnName="Xor gate"
              nnAddress={account?.decodedAddress ?? '0x00'}
              nnImage={XorImage}
              linkToDemo="/account/xor-demo-nn"
              inputs={2}
              outputs={1}
            />
            <CardNNData
              nnName="Doodles"
              nnAddress={account?.decodedAddress ?? '0x00'}
              linkToDemo="/account/doodle-demo-nn"
              nnImage={DoodleImage}
              inputs={784}
              outputs={3}
            />

            {/* <CardNNData nnName="Xor gate" nnAddress={account?.decodedAddress ?? "0x00"} linkToDemo="/account/xor-demo" />
                            <CardNNData nnName="Xor gate" nnAddress={account?.decodedAddress ?? "0x00"} linkToDemo="/account/xor-demo" />
                            <CardNNData nnName="Xor gate" nnAddress={account?.decodedAddress ?? "0x00"} linkToDemo="/account/xor-demo" />
                            <CardNNData nnName="Xor gate" nnAddress={account?.decodedAddress ?? "0x00"} linkToDemo="/account/xor-demo" />
                            <CardNNData nnName="Xor gate" nnAddress={account?.decodedAddress ?? "0x00"} linkToDemo="/account/xor-demo" />
                            <CardNNData nnName="Xor gate" nnAddress={account?.decodedAddress ?? "0x00"} linkToDemo="/account/xor-demo" />
                            <CardNNData nnName="Xor gate" nnAddress={account?.decodedAddress ?? "0x00"} linkToDemo="/account/xor-demo" /> */}
          </div>
        </>
      ) : (
        <div className="account__subscriptions-container">
          <Subscriptions onSubscription={userDoASubscription} />
        </div>
      )}
    </div>
  );
}
