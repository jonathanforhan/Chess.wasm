import '../App.css';

export default function SideBar(props: any) {
  return(
    <div className='SideBar'>
      <div className='SideBarMoves'/>
      <div className='ButtonWrapper'>
        <button className='UndoButton'
          onClick={() => {
            props.undo();
            props.undo();
          }}
        > Undo
        </button>
        <button className='ResetButton'
          onClick={() => props.reset() }
        > Reset
        </button>
      </div>
    </div>
  )
}

