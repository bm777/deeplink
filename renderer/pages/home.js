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
                <div className=' h-[90%] my-auto w-[40%] flex items-center justify-center relative'>
                  <div className=' flex justify-center gap-3 w-[300px] h-[300px] absolute  animate-pulse'>
                    {
                      Array.from({length: 10}, () => Math.floor(Math.random() * 10)).map((num, i) => (
                        <div className=' bg-black w-[10px]  rounded-full h-[300px]'></div>
                      ))
                    } 
                  </div>
                  <div className=' flex-col flex items-center justify-center gap-3 w-[300px] h-[300px] absolute animate-pulse'>
                    {
                      Array.from({length: 10}, () => Math.floor(Math.random() * 10)).map((num, i) => (
                        <div className=' bg-black h-[10px]  rounded-full w-[300px]'></div>
                      ))
                    } 
                  </div>
                  <div className=' flex-col flex items-center bg-black rounded-xl justify-center w-[250px] h-[250px] absolute'>
                    <div className=' flex-col flex items-center bg-black border-[10px] border-white rounded-md justify-center w-[230px] h-[230px] absolute'>
                    {
                      checking ?
                      <p className='text-[30px] text-white '>checking</p>
                      :
                      <p className='text-[70px] text-white '>GPU</p>
                    }
                    </div>
                  </div>
                </div>
                <div className=' h-[90%] my-auto'></div>
                <div className=' h-[90%] my-auto flex flex-col justify-center'>

                  <div className=' border border-[#00000017] h-[80px] w-[150px] shadow-xl shadow-[#00000007] rounded-xl flex justify-center pt-2 relative'>
                    <div className=' text-2xl font-medium'>24 GB </div>
                    <div className=' absolute bottom-2 flex'>
                      <span className='text-[#45363D]'>RAM</span>
                    </div>
                  </div>

                  <div className=' border border-[#00000017] h-[80px] w-[150px] shadow-xl shadow-[#00000007] rounded-xl flex justify-center pt-2 relative mt-3'>
                    <div className=' text-2xl font-medium'>6 GB </div>
                    <div className=' absolute bottom-2 flex'>
                      <span className='text-[#45363D]'>GPU</span>
                    </div>
                  </div>

                  <div className=' border border-[#00000017] h-[80px] w-[150px] shadow-xl shadow-[#00000007] rounded-xl flex justify-center pt-2 relative mt-3'>
                    <div className=' text-2xl font-medium'>32 Mbps </div>
                    <div className=' absolute bottom-2 flex'>
                      <span className='text-[#45363D]'>SPEED</span>
                    </div>
                  </div>
                </div>
              </div>
              <div className=' h-[100px]'></div>
            </div>
          </div>
        }
        {
          tab === 'Inference' &&
          <div className='flex flex-col w-[750px]'>
            
          </div>
        }
        {
          tab === 'Usage' &&
          <div className='flex flex-col w-[750px]'>
            
          </div>
        }
        
      </div>
    </main>
  )
}