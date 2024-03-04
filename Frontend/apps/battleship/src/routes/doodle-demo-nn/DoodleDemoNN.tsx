import { DOODLE_NN_CONTRACT } from '@/app/consts';
import { useEffect, useState } from 'react';
import { Subscriptions, CardNNData, Sketch } from '@/components';
import { useContractUtils } from '@/app/hooks';
import { TemplateAlertOptions, useAccount, useAlert } from '@gear-js/react-hooks';
import { XorNumSelection, ButtonPredict } from '@/components';
import './DoodleDemoNN.scss';

interface Fraction { num: number, den: number };

export default function DoodleDemoNN() {
  const { account } = useAccount();
  const alert = useAlert();
  const { readState } = useContractUtils();
  const [userIsSubscribed, setUserIsSubscribed] = useState(false);
  const [data, setData] = useState<number[]>([]);
  const [prediction, setPrediction] = useState("");

  const saveData = (data: number[]) => {
    console.log("NEW DATA:");
    console.log(data);
    setData(data);
  }

  const userDoASubscription = () => {
    setUserIsSubscribed(true);
  };

  const handlePredictionMessage = async () => {
    setPrediction("");
    if (!account) {
        alert.error("Account is not started");
        return;
    }

    const {userLastPrediction}: any = await readState(
        DOODLE_NN_CONTRACT.programId,
        DOODLE_NN_CONTRACT.programMetadata,
        {
            UserLastPrediction: account.decodedAddress
        }
    );

    console.log("User last prediction");
    
    console.log(userLastPrediction);

    const probabilities = userLastPrediction.map((probability: string) => {
      const [num, den] = probability.split(' / ');
      return Number(num) / Number(den);
    });

    console.log("Probabilities in number: ");
    console.log(probabilities);
    
    const maxValue = Math.max(probabilities);

    console.log("Max value: ", maxValue);
    

    let indexMaxValue = 0;
    probabilities.forEach((value: any, index: any) => {
      if (value == maxValue) indexMaxValue = index;
    });

    console.log('Index of result: ',indexMaxValue);
    

    let predictResult = "";
    if (indexMaxValue == 0) {
      predictResult = "Cat";
      setPrediction("Cat!");
    } else if (indexMaxValue == 1) {
      predictResult = "Train";
      setPrediction("Train!");
    } else {
      predictResult = "Rainbow";
      setPrediction("Rainbow!");
    }
    

    // const [ num, den ] = userLastPrediction.split(' / ');

    // const probability = num / den;
    
    // if (probability >= 0.9) {
    //     setPrediction(1);
    // } else {
    //     setPrediction(0);
    // }

    const alertOptions: TemplateAlertOptions = {
        title: "NeuroShark",
      };

    alert.info(`probability of it being a ${predictResult}: ${(maxValue) * 100}%`, alertOptions);
  };

  // const input_data = (values: number[]): Fraction[] => {
  //   let fractions: Fraction[] = [];
  //   values.forEach((value) => {
  //     let value_string = value.toString();
  //     if (!value_string.includes(".")) {
  //       fractions += fraction_formatter(value, 1) + "\n";
  //     } else {
  //       let [integer, decimal] = value_string.toString().split(".");
  
  //       if (decimal.length > 12) {
  //         decimal = decimal.substring(0, 12);
  //       }
  
  //       while (decimal.charAt(0) == "0") {
  //         decimal = decimal.substring(1, decimal.length);
  //       }
  
  //       if (integer.charAt(0) == "-" && integer.charAt(1) == "0") {
  //         integer = "-";
  //       } else if (integer.charAt(0) == "0") {
  //         integer = "";
  //       }
  
  //       fractions += fraction_formatter(integer + decimal, 1000000000000) + "\n";
  //     }
  //   })
  //   console.log(fractions);
  // };

  const input_data = (values: number[]): Fraction[] => {
    let fractions: Fraction[] = [];
    values.forEach((value) => {
      let value_string = value.toString();
      if (!value_string.includes(".")) {
        fractions.push(fraction_formatter(Number(value), 1));
      } else {
        let [integer, decimal] = value_string.toString().split(".");
  
        if (decimal.length > 12) {
          decimal = decimal.substring(0, 12);
        }
  
        while (decimal.charAt(0) == "0") {
          decimal = decimal.substring(1, decimal.length);
        }
  
        if (integer.charAt(0) == "-" && integer.charAt(1) == "0") {
          integer = "-";
        } else if (integer.charAt(0) == "0") {
          integer = "";
        }
  
        fractions.push(fraction_formatter(Number(integer + decimal), 1000000000000));
      }
    })
    console.log(fractions);
    return fractions;
  };
  
  const fraction_formatter = (num: number, den: number): Fraction => {
    return (
    {
      num,
      den
    }
    );
  }

  const dataToSendToNN = () => {
    return { Predict:  input_data(data)};
  };

  const isSubscribed = async () => {
    if (!account) {
      alert.error('Account is not initialized');
      return;
    }

    setUserIsSubscribed(true);

    // Contracts does not have the same actions and events, so, doodle contract will not have 
    // the user subscription, after fixed, we can discomment these part 

    // const { userIsSubscribed }: any = await readState(DOODLE_NN_CONTRACT.programId, DOODLE_NN_CONTRACT.programMetadata, {
    //   UserIsSubscribed: account.decodedAddress,
    // });

    // setUserIsSubscribed(userIsSubscribed);

    // if (!userIsSubscribed) {
    //   return;
    // }
  };

  useEffect(() => {
    async function test() {
      await isSubscribed();
    }

    test();
  }, [account, userIsSubscribed, userDoASubscription]);

  return (
    <div className="doodle-demo-nn">
      {userIsSubscribed ? (
        <div className="doodle-demo-nn__work-space">
          <div className='doodle-demo-nn__description-container'>
            <h1 className="doodle-demo-nn__tittle">doodle recognition</h1>
            <p className='doodle-demo-nn__text-description'>Recognize between a train, a cat or a rainbow!</p>
          </div>
          <div className="doodle-demo-nn__container">
            <Sketch onSaveDataOfSketch={saveData}/>
          </div>
          <p className='doodle-demo-nn__prediction-text'>I think it's a: {prediction}</p>
          <ButtonPredict
            onPredict={handlePredictionMessage}
            available={true}
            payload={dataToSendToNN()}
            programId={DOODLE_NN_CONTRACT.programId}
            programMetadata={DOODLE_NN_CONTRACT.programMetadata}
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
