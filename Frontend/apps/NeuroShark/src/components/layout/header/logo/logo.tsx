import { Link } from 'react-router-dom';
import { ReactComponent as NeuroSharkImagoType } from  '@/assets/images/imagotype.svg'
import './logo.scss';

const  Logo = () => {
  return (
    <Link to="/" className='header__a-logo'>
      <NeuroSharkImagoType className="imagotype" />
    </Link>
  );
}

export { Logo };
