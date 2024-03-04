import { Link } from 'react-router-dom';
import './CardNNData.scss';

interface CardNNDataProps {
  nnName: string;
  nnImage: any;
  nnAddress: `0x${string}`;
  linkToDemo: string;
  inputs: number;
  outputs: number;
}

export function CardNNData({ nnName, nnImage, nnAddress, linkToDemo, inputs, outputs }: CardNNDataProps) {
  function shortenAddress(addrress: `0x${string}`): string {
    if (addrress.length > 20) {
      return addrress.slice(0, 10) + '...' + addrress.slice(-10);
    }
    return addrress;
  }

  return (
    <div className="card-nn-data">
      <p className="card-nn-data__title">{nnName}</p>
      <div className="card-nn-data__image-container">
        <img src={nnImage} alt="xor image icon" className="card-nn-data__image" />
      </div>
      <p className="card-nn-data__id card-nn-data--p-w-50">{shortenAddress(nnAddress)}</p>
      <div className="card-nn-data__io-container">
        <p className="card-nn-data__id card-nn-data--p-w-50">inputs: {inputs}</p>
        <p className="card-nn-data__id card-nn-data--p-w-50">outputs: {outputs}</p>
      </div>
      <Link to={linkToDemo} className="card-nn-data__button">
        Try it!
      </Link>
    </div>
  );
}
