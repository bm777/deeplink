import { useEffect, useState } from 'react'
import Head from 'next/head'
import Link from 'next/link'
import Image from 'next/image'

let ipcRenderer
if(typeof window !== "undefined" && window.process && window.process.type === "renderer"){
  ipcRenderer = window.require('electron').ipcRenderer
}

export default function HomePage() {
  const [message, setMessage] = useState('No message found')

  useEffect(() => {
    
    const listener = async (event, arg) => {
      ipcRenderer.send('message', 'Hello from renderer')
      ipcRenderer.on('message-reply', (event, arg) => {
        console.log("DEBUG: ipcRenderer <listener>")
        setMessage(arg)
      })
    }

    listener()

    return () => { ipcRenderer.removeAllListeners('message') }
  }, [])


  return (
    <main>
      <Head>
        <title>Home - Nextron (basic-lang-javascript)</title>
      </Head>
      <div>
        <p>
          ⚡ Electron + Next.js ⚡ -
          <Link href="/next">
            <a>Go to next page</a>
          </Link>
        </p>
        <Image
          src="/images/logo.png"
          alt="Logo image"
          width="256px"
          height="256px"
        />
      </div>
      <div>
        <button className=' bg-red-500'

        >
          Test IPC
        </button>
        <p>{message}</p>
      </div>
    </main>
  )
}