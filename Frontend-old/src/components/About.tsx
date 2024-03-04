import DesignSVG from '../../public/design.svg';
import BuildPNG from '../../public/build.svg'
import DeploySVG from '../../public/deployico.svg';
import Image from 'next/image';

export const About = () => {
  return (
    <div className='w-full flex flex-col text-center justify-center items-center mt-20'>
      <h1 className='text-primary-main text-5xl font-bold'>What is NeuroShark?</h1>
      <div className='text-4xl w-4/5 text-center mx-auto mt-5'>NeuroShark is a web3 service whose purpose is to allow users to address less frequent problems by using neural networks on the blockchain.</div>
      <div className='w-full flex mt-20 justify-evenly'>
        <div className='p-8 border-2 rounded-lg flex flex-col justify-center items-center'>
          <p className='text-3xl mb-4'>Desing</p>
          <Image src={DesignSVG} alt='Design' width={113} height={113} />
        </div>
        <div className='p-8 border-2 rounded-lg flex flex-col justify-center items-center'>
          <p className='text-3xl mb-4'>Build</p>
          <Image src={BuildPNG} alt='Build' width={113} height={113} />
        </div>
        <div className='p-8 border-2 rounded-lg flex flex-col justify-center items-center'>
          <p className='text-3xl mb-4'>Deploy</p>
          <Image src={DeploySVG} alt='Build' width={95} height={95} />
        </div>
      </div>
    </div>
  )
}
