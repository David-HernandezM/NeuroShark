import { Entries } from '../types';
import  PolkadotSVG from './assets/polkadot.svg';

const WALLET = {
  'polkadot-js': { name: 'Polkadot JS', SVG: PolkadotSVG },
};

const WALLETS = Object.entries(WALLET) as Entries<typeof WALLET>;

export { WALLET, WALLETS };