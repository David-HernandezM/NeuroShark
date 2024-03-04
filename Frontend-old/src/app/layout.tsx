import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import { Navbar } from '@/components'
import Icon from '../../public/icon.svg'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'NeuroShark',
  description: "NeuroShark's website office",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Navbar mode='LOGGED' />
        {children}
      </body>
    </html>
  )
}
