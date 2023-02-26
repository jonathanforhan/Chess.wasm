import '../App.css';

export default function SideBar(props: any) {
  return(
    <div className='SideBar'>
      <div className='SideBarMoves'/>
      <div className='ButtonWrapper'>
        <button className='UndoButton'
          onClick={() => {
            props.safeGameMutate((g: any) => g.undo());
            props.chessboardRef.current?.clearPremoves();
            clearTimeout(props.currentTimeout);
          }}
        > Undo
        </button>
        <button className='ResetButton'
          onClick={() => {
            props.safeGameMutate((g: any) => g.reset());
            props.chessboardRef.current?.clearPremoves();
            clearTimeout(props.currentTimeout);
          }}
        > Reset
        </button>
      </div>
    </div>
  )
}

