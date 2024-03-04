import { ButtonSubscribe, SubscriptionCost } from "../ButtonSubscribe/ButtonSubscribe";
import { useContractUtils, useVoucherUtils } from "@/app/hooks";
import { ReactComponent as FreeSubscriptionImage } from  '@/assets/images/plan_free_image.svg';
import { ReactComponent as BasicSubscriptionImage } from  '@/assets/images/plan-basic-image.svg';   
import { ReactComponent as UltimateSubscriptionImage } from  '@/assets/images/plan-ultimate-image.svg';
import "./CardSubscription.scss";

export interface CardSubscriptionProps {
    subscriptionCost: SubscriptionCost,
    onUserSubscribe: any
    descriptions: string[],
    isAvailable: boolean,
    isMain?: boolean
}

export function CardSubscription({ subscriptionCost, onUserSubscribe, descriptions, isAvailable, isMain }: CardSubscriptionProps) {
    const imageCards = {
        "free": <FreeSubscriptionImage className="card-subscription__title-image"/>,
        "basic": <BasicSubscriptionImage className="card-subscription__title-image"/>,
        "ultimate": <UltimateSubscriptionImage className="card-subscription__title-image"/>
    }

    const tittleCards = {
        "free": "Free",
        "basic": "Basic",
        "ultimate": "Ultimate"
    }

    return (
        <>
            <div className={`card-subscription${isMain ? " card-subscription--gap30" : ""}`}>
                <div className="card-subscription__title-container">
                    <h2 className="card-subscription__title">{tittleCards[subscriptionCost]}</h2>
                    {
                        imageCards[subscriptionCost]
                    }
                </div>
                <div className={`card-subscription__descriptions-container${isMain ? " card-subscription--gap30" : ""}`}>
                    {
                        descriptions.map((description, index) => {
                            return <p className="card-subscription__description" key={index}>{description}</p>
                        })
                    }
                </div>
                <ButtonSubscribe cost={subscriptionCost} onSubscribe={onUserSubscribe} available={isAvailable} />
            </div>
        </>
    );
}