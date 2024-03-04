import { CardSubscription } from '../CardSubscription/CardSubscription';
import './Subscriptions.scss';

interface SubscriptionsProps {
  onSubscription?: any
}

export function Subscriptions({ onSubscription }: SubscriptionsProps) {
  const freeDescriptions = ['Two free neural networks'];

  const basicDescriptions = ['Access to basic neural networks', 'Three Custom Neuronal Networks'];

  const ultimateDescriptions = ['Access to all neural networks', 'Six Custom Neuronal Networks'];

  return (
    <div className="subscriptions">
      <h2 className="subscriptions-tittle">Check out our subscriptions!</h2>
      <div className="subscriptions-container">
        <CardSubscription
          subscriptionCost="free"
          onUserSubscribe={onSubscription}
          descriptions={freeDescriptions}
          isAvailable={true}
        />
        <CardSubscription
          subscriptionCost="ultimate"
          onUserSubscribe={onSubscription}
          descriptions={ultimateDescriptions}
          isAvailable={false}
          isMain={true}
        />
        <CardSubscription
          subscriptionCost="basic"
          onUserSubscribe={onSubscription}
          descriptions={basicDescriptions}
          isAvailable={false}
        />
      </div>
    </div>
  );
}
