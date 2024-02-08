import { useEffect, useState } from 'react'
import Head from 'next/head'
import Btn from '../components/btn'
import Card from '../components/card'
import GPU from '../components/gpu'
import Token from '../components/token'
import Model from '../components/model'

let ipcRenderer
if(typeof window !== "undefined" && window.process && window.process.type === "renderer"){
  ipcRenderer = window.require('electron').ipcRenderer
}

export default function HomePage() {
  const [message, setMessage] = useState('No message found')
  const [tab, setTab] = useState('Check')
  const [checking, setChecking] = useState(false)

  // useEffect(() => { 
  //   const listener = async (event, arg) => {
  //     ipcRenderer.send('message', 'Hello from renderer')
  //     ipcRenderer.on('message-reply', (event, arg) => {
  //       console.log("DEBUG: ipcRenderer <listener>")
  //       setMessage(arg)
  //     })
  //   }
  //   listener()

  //   return () => { ipcRenderer.removeAllListeners('message') }
  // }, [])

  // when the user clicks on the button, the tab state changes
  const handleClick = (e) => {
    setTab(e.target.textContent)
    // set object enable to true if the button is clicked
    e.target.enabled = true
  }

  return (
    <main className='w-full h-screen'>
      <Head>
        <title>deeplink</title>
      </Head>

      <div className='flex w-full h-full'>
        {/* left */}
        <div className='flex flex-col h-full w-[250px] bg-[#EED6EA]'>
          <div className=' bg-[#E1C9DD] flex mx-3 mt-7 rounded py-[2px] px-2 text-[#6C5468]'>@IP</div>
          <div className='flex-1 pt-3 relative'>
            <Btn text="Check" handleClick={handleClick} enabled={"Check" === tab}/>
            <Btn text="Inference" handleClick={handleClick} enabled={"Inference" === tab} />
            <div className='absolute w-full bottom-3'>
              <Btn text="GPU Usage" handleClick={() => {}} enabled={"" === tab} extra={45} />
            </div>
          </div>
        </div>

        {/* center or right */}
        {
          tab === 'Check' &&
          <div className='flex flex-col w-[750px]'>
            <div className='flex items-center pl-7 h-[100px] text-3xl font-medium w-full relative'>
              Check your hardware
              <p className='text-lg font-normal absolute bottom-0'>Your GPU can support inference</p>
            </div>
            <div className='flex-1 flex flex-col '>
              <div className='flex-1 flex justify-center gap-10'>
                <GPU checking={checking}></GPU>
                <div className=' h-[90%] my-auto'></div>
                <div className=' h-[90%] my-auto flex flex-col justify-center gap-4'>

                  <Card title="24 GB" subtitle="RAM"/>
                  <Card title="6 GB" subtitle="GPU"/>
                  <Card title="32 Mbps" subtitle="SPEED"/>
                  
                </div>
              </div>
              <div className=' h-[100px]'></div>
            </div>
          </div>
        }
        {
          tab === 'Inference' &&
          <div className='flex flex-col w-[750px]'>
            <div className='flex items-center pl-7 h-[100px] text-3xl font-medium w-full relative'>
              Inference on your device
              <p className='text-lg font-normal absolute bottom-0'>We count the input and output token generated this device.</p>
            </div>
            <div className='flex-1 flex flex-col '>
              <div className='flex-1 flex justify-center gap-10'>

                <div className=' h-[90%] my-auto flex  gap-4'>

                  <Token title="12.5 k" subtitle="input otken"/>
                  <Token title="1.4 M" subtitle="output token"/>
                  <Model title="Mistral 8x7B" subtitle="model"/>
                  
                </div>
              </div>
              <div className=' h-[100px]'></div>
            </div>
          </div>
        }
        
      </div>
    </main>
  )
}