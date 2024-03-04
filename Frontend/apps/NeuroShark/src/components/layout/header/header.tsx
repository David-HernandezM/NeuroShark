import React from 'react';
import { Logo } from './logo';
import { AccountInfo } from './account-info';
import { Link, NavLink } from 'react-router-dom';
import './header.scss';

type Props = {
  isAccountVisible: boolean;
};

export function Header({ isAccountVisible }: Props) {
  const [isMenuOpen] = React.useState(false);

  return (
    <header className="header">
      <Logo />
      <div className='header__container'>
        <nav className='header__nav'>
          <ul className='header__nav-ul'>
            <li className='header__nav-ul-li'>
              <Link to="about">About</Link>
            </li>
            <li className='header__nav-ul-li'>
            <Link to="/#how-to-start">How to start</Link>
            </li>
            <li className='header__nav-ul-li'>
            <Link to={"/#faq"}>FAQ</Link>
            </li>
            <li className='header__nav-ul-li'>
              <Link to="/account">Accunt</Link>
            </li>
          </ul>
        </nav>
      
        {isAccountVisible && <AccountInfo />}
      </div>      
    </header>
  );
}
