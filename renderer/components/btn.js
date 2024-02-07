import Inference from "./svg/inference"
import Check from "./svg/check"
import Usage from "./svg/usage"

export default function Btn({ text, handleClick, enabled}) {
    if(enabled) console.log(text, enabled)
    if (enabled) { 
        return (
            <div className='bg-[#E1C9DD] flex items-center mx-3 mt-[1px] rounded py-[5px] px-2 text-[#6C5468] hover:cursor-pointer transition duration-300'>
                { text === "Check" && <Check /> }
                { text === "Inference" && <Inference /> }
                { text === "Usage" && <Usage /> }
                <span className='text-[#45363D] text-sm font-medium ml-2'>{text}</span>
            </div>
            )
    } else { 
        return (
            <div onClick={handleClick} className='flex items-center mx-3 mt-[1px] rounded py-[5px] px-2 text-[#6C5468] hover:cursor-pointer hover:bg-[#E1C9DD]'>
                { text === "Check" && <Check /> }
                { text === "Inference" && <Inference /> }
                { text === "Usage" && <Usage /> }
                <span className='text-[#45363D] text-sm font-medium ml-2'>{text}</span>
            </div>
            )
    }

}