
import { useState } from 'react';
import './XorNumSelection.scss';

interface XorNumSelectionProps {
    onNumChanged?: any,
    changeValueTo?: number
}

export function XorNumSelection({ onNumChanged, changeValueTo }: XorNumSelectionProps) {
    const [num, setNum] = useState(changeValueTo ? changeValueTo : 0);

    const toogle = () => {
        let actualValue = num;
        if (num == 0) {
            actualValue = 1;
            setNum(1);
        } else {
            actualValue = 0;
            setNum(0);
        }

        if (onNumChanged) {
            onNumChanged(actualValue);
        }
    }

    return (
        <p className='xor-num-selection' onClick={toogle}>{ changeValueTo ? changeValueTo : num }</p>
    )
}