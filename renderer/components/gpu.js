export default function GPU({checking}) {
    return (
        <div className=' h-[90%] my-auto w-[40%] flex items-center justify-center relative'>
          <div className=' flex justify-center gap-3 w-[300px] h-[300px] absolute  animate-pulse'>
            {
              Array.from({length: 10}, () => Math.floor(Math.random() * 10)).map((num, i) => (
                <div className=' bg-black w-[10px]  rounded-full h-[300px]' key={"tab"+i}></div>
              ))
            } 
          </div>
          <div className=' flex-col flex items-center justify-center gap-3 w-[300px] h-[300px] absolute animate-pulse'>
            {
              Array.from({length: 10}, () => Math.floor(Math.random() * 10)).map((num, i) => (
                <div className=' bg-black h-[10px]  rounded-full w-[300px]' key={"tab-"+i}></div>
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
    )
}