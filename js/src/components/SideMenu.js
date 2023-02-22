import '../App.css';

export default function SideMenu(props) {
  return(
    <>
      <div className='MenuWrapper bg-slate-800 w-[40vh] justify-center'>
        <div className='Moves h-[85%]'/>
        <div className='ButtonWrapper grid'>
          <button className='UndoButton rounded-sm bg-gray-50 mx-[20%] text-xl'
            onClick={() => {
              props.safeGameMutate(g => g.undo());
              props.chessboardRef.current?.clearPremoves();
              clearTimeout(props.currentTimeout);
            }}
          > Undo
          </button>
          <button className='ResetButton rounded-sm bg-gray-50 mx-[20%] my-[1rem] text-xl'
            onClick={() => {
              props.safeGameMutate(g => g.reset());
              props.chessboardRef.current?.clearPremoves();
              clearTimeout(props.currentTimeout);
            }}
          > Reset
          </button>
        </div>
      </div>
    </>
  )
}
