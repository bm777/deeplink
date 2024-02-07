import { useEffect, useState } from 'react'
import Head from 'next/head'
import Btn from '../components/btn'

let ipcRenderer
if(typeof window !== "undefined" && window.process && window.process.type === "renderer"){
  ipcRenderer = window.require('electron').ipcRenderer
}

export default function HomePage() {
  const [message, setMessage] = useState('No message found')
  const [tab, setTab] = useState('Check')

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

  // when the user clicks on the button, the tab state changes
  const handleClick = (e) => {
    setTab(e.target.textContent)
  }

  return (
    <main className='w-full h-screen'>
      <Head>
        <title>deeplink</title>
      </Head>

      <div className='flex w-full h-full'>
        {/* left */}
        <div className='flex flex-col h-full w-[25%] bg-[#EED6EA]'>
          <div className=' bg-[#E1C9DD] flex mx-3 mt-7 rounded py-[2px] px-2 text-[#6C5468]'>@IP</div>

          <div className='flex-1'>
            <Btn text="Check" onClick={handleClick} enabled={"Check" === tab}/>
            <Btn text="Inference" onClick={handleClick} enabled={"Inference" === tab} />
            <Btn text="Usage" onClick={handleClick} enabled={"Usage" === tab} />
          </div>

        </div>

        {/* center or right */}
        <div className='flex w-full'>
          right
        </div>
        
      </div>
    </main>
  )
}