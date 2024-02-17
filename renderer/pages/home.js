import { useEffect, useState } from 'react'
import Head from 'next/head'
import Btn from '../components/btn'
import Card from '../components/card'
import GPU from '../components/gpu'
import Token from '../components/token'
import Model from '../components/model'
import { checkHardware, getIpAddress } from './api/methods'

let ipcRenderer
if(typeof window !== "undefined" && window.process && window.process.type === "renderer"){
  ipcRenderer = window.require('electron').ipcRenderer
}

export default function HomePage() {
  const [message, setMessage] = useState('No message found')
  const [tab, setTab] = useState('Check')
  const [checking, setChecking] = useState(false)
  const [activeInference, setActiveInference] = useState(false)
  const [gpu, setGpu] = useState("")
  const [ram, setRam] = useState("")
  const [ip, setIp] = useState("")
  const [speed, setSpeed] = useState("")
  const [inputToken, setInputToken] = useState("")
  const [outputToken, setOutputToken] = useState("")
  const [model, setModel] = useState("")
  const [error, setError] = useState("")


  useEffect(() => { 
    const check = async () => {
      setChecking(true)
      const data = await checkHardware()
      const result = data["data"]
      console.log(result)
      if (result.error) {
        setError(result.error)
        setChecking(false)
        return
      }
      setGpu( result.gpu.includes("Chipset Model") ? result.gpu.split("Chipset Model: ")[1] : result.gpu )
      setRam(Math.ceil(result.ram / ( 1024 * 1024)))
      setSpeed(result.speed)
      setInputToken(result.inputToken)
      setOutputToken(result.outputToken)
      setModel(result.model)
      if (result.ram >= 8388608 && result.gpu.includes("Chipset Model")) {
        setActiveInference(true)
      }
      // set the ip address
      const _ip = await getIpAddress()
      if (_ip.error) {
        setError(_ip.error)
        setChecking(false)
        return
      }
      setIp(_ip.ip)

      // get internet speed
      if (navigator.connection) {
        setSpeed(navigator.connection.downlink)
      }
      setChecking(false)
    }

    check()
  }, [])

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

      <div className='flex w-full h-full '>
        {/* left */}
        <div className='flex flex-col h-full w-[250px] bg-[#EED6EA]'>
          <div className=' bg-[#E1C9DD] flex mx-3 mt-7 rounded py-[2px] px-4 text-[#6C5468] font-medium'>{ip}</div>
          <div className='flex-1 pt-3 relative'>
            <Btn text="Check" handleClick={handleClick} enabled={"Check" === tab}/>
            <Btn text="Inference" handleClick={handleClick} enabled={"Inference" === tab} active={activeInference}/>
            <div className='absolute w-full bottom-3'>
              <Btn text="GPU Usage" handleClick={() => {}} enabled={"" === tab} extra={45} />
            </div>
          </div>
        </div>

        {/* center or right */}
        {
          tab === 'Check' &&
          <div className='flex flex-col w-[750px] '>
            <div className='flex items-center pl-7 h-[100px] text-3xl font-medium w-full relative'>
              Check your hardware
              <p className='text-lg font-normal absolute bottom-0'>Your GPU can support inference</p>
            </div>
            <div className='flex-1 flex flex-col '>
              <div className='flex-1 flex justify-center gap-10'>
                <GPU checking={checking}></GPU>
                <div className=' h-[90%] my-auto'></div>
                <div className=' h-[90%] my-auto flex flex-col justify-center gap-4'>

                  <Card title={ram ? ram : "--"} subtitle="RAM" unit="GB"/>
                  <Card title={gpu ? gpu : "--"} subtitle="GPU" unit=""/>
                  <Card title={speed ? speed : "--"} subtitle="SPEED" unit="Mbps"/>
                  
                </div>
              </div>
              <div className=' h-[100px]'></div>
            </div>
          </div>
        }
        {
          (tab === 'Inference' && activeInference) &&
          <div className='flex flex-col w-[750px] '>
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
        {
          (tab === 'Inference' && !activeInference) &&
          <div className='flex flex-col w-[750px] '>
            <div className='flex items-center pl-7 h-[100px] text-3xl font-medium w-full relative'>
              Cannot make inference on your device
              <p className='text-lg font-normal absolute bottom-0'>Upgrade your GPU hardware in order to run models.</p>
            </div>
          </div>
        }
        
      </div>
    </main>
  )
}