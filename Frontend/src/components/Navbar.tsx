import Image from 'next/image'
import Logo from '../../public/logo.svg';
import { Wallet } from '@/Icons';

interface Props {
  mode: 'LOGGED' | 'NOT-LOGGED'
}

export const Navbar = ({ mode }: Props) => {
  return (

    <nav className="bg-background">
      <div className="max-w-screen-xxl flex flex-wrap items-center justify-between py-2 px-4">
        <a href="/" className="flex items-center">
          <Image src={Logo} alt='Inicio' width={200} />
        </a>
        <div className="flex space-x-8" id="navbar-default">
          {mode === 'LOGGED' &&
            <><div>
              <p>Balance:</p>
              <p>78.022 TVARA</p>
            </div>
              <div className='flex justify-center items-center space-x-2 rounded-full px-4 bg-gray-600'>
                <div className='w-5 h-5 rounded-full bg-red-500'></div>
                <p>David</p>
              </div>
              <button type="button" className="bg-primary-main hover:bg-primary-100 font-medium rounded-full px-5 py-2.5 text-center">BUILD</button></>}
          {mode === 'NOT-LOGGED' && <button type="button" className=" bg-black hover:bg-primary-100 ease-in duration-200 font-medium rounded-full px-5 py-2.5 text-center flex items-center space-x-2">
            <Wallet />
            <p>CONNECT</p>
          </button>}
        </div>
      </div>
    </nav>

  )
}
