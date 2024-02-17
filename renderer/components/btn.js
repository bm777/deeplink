import Inference from "./svg/inference"
import Check from "./svg/check"
import Usage from "./svg/usage"

export default function Btn({ text, handleClick, enabled, extra, active}) {

    if (enabled) { 
        return (
            <div className='bg-[#E1C9DD] flex items-center mx-3 mt-[1px] rounded py-[5px] px-2 text-[#6C5468] hover:cursor-pointer transition duration-300'>
                { text === "Check" && <Check /> }
                { (text === "Inference" ) && <Inference /> }
                { text === "Usage" && <Usage /> }
                {  <span className='text-[#45363D] text-sm font-medium ml-2'>{text}</span> }
            </div>
            )
    } else { 
        return (
            <div onClick={handleClick} className='flex items-center mx-3 mt-[1px] rounded py-[5px] px-2 text-[#6C5468] hover:cursor-pointer hover:bg-[#E1C9DD] relative transi duration-500'>
                { text === "Check" && <Check /> }
                { text === "Inference" && <Inference /> }
                { text === "GPU Usage" && <Usage /> }
                <span className='text-[#45363D] text-sm font-medium ml-2'>{text}</span>
                { extra && <span className='absolute right-0 bg-[#E1C9DD] rounded-full font-semibold text-[#086AE3] px-[8px] py-1 text-xs'>{extra}%</span> }
            </div>
            )
    }

}