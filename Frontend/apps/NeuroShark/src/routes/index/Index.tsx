import { Link } from 'react-router-dom';
import IndexImage from '@/assets/images/image_index.png';
import './Index.scss';

export default function Index() {
    return (
        <div className='index-container'>
            <h2 className='index-container__title'>What is NeuroShark?</h2>
            <p className='index-container__description'>NeuroShark is a web3 service whose purpose is to allow users to address less frequent problems by using neural networks on the blockchain.</p>
            <Link to='account' className='index-container__button-account'>Build!</Link>
        </div>
    );
}