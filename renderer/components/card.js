export default function Card({ title, subtitle }) {
  return (
    <div className=' border border-[#00000017] h-[80px] w-[150px] shadow-xl shadow-[#00000007] rounded-xl flex justify-center pt-2 relative hover:cursor-pointer'>
        <div className=' text-2xl font-medium'>{title}</div>
        <div className=' absolute bottom-2 flex'>
            <span className='text-[#45363D]'>{subtitle}</span>
        </div>
    </div>
  )
}